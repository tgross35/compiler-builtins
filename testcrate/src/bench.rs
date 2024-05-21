use core::cell::RefCell;

use alloc::vec::Vec;
use compiler_builtins::float::Float;

/// Fuzz with these many items to ensure equal functions
pub const CHECK_ITER_ITEMS: u32 = 10_000;
/// Benchmark with this many items to get a variety
pub const BENCH_ITER_ITEMS: u32 = 500;

/// Still run benchmarks but don't check correctness between compiler-builtins and
/// system functions
pub const SKIP_SYS_CHECKS: &[&str] = &[
    // FIXME: some sort of precision error (tested on aarch64)
    "extend_f16_f32",
    "trunc_f32_f16",
    // We return -1, system functions on x86 return -2
    "cmp_f128_gt",
    // FIXME: rounding error
    // <https://github.com/rust-lang/compiler-builtins/issues/616#issuecomment-2121060728>
    "mul_f128",
    // System symbols do the wrong thing
    // <https://github.com/rust-lang/compiler-builtins/issues/617>
    "trunc_f64_f16",
];

/// Create a comparison of the system symbol, compiler_builtins, and optionally handwritten
/// assembly.
///
/// `asm!` gets turned into global assembly, more or less a naked function.
#[macro_export]
macro_rules! float_bench {
    (
        // Name of this benchmark
        name: $name:ident,
        // The function signature to be tested
        sig: ($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty,
        // Path to the crate in compiler_builtins
        crate_fn: $crate_fn:path,
        // Name of the system symbol
        sys_fn: $sys_fn:ident,
        // Meta saying whether the system symbol is available
        sys_available: $sys_available:meta,
        // Assembly implementations, if any.
        asm: [
            $(
                #[$asm_meta:meta] {
                    $($asm_tt:tt)*
                }
            );*
            $(;)?
        ]
        $(,)?
    ) => {paste::paste! {
        #[allow(dead_code)]
        extern "C" {
            /// Binding for the system function
            fn $sys_fn($($arg: $arg_ty),*) -> $ret_ty;
        }

        fn $name(c: &mut Criterion) {
            use core::hint::black_box;
            use compiler_builtins::float::Float;
            use $crate::bench::BenchType;

            #[inline(never)] // equalize with external calls
            fn crate_fn($($arg: $arg_ty),*) -> $ret_ty {
                $crate_fn( $($arg),* )
            }

            #[inline(always)] // already a branch
            #[cfg($sys_available)]
            fn sys_fn($($arg: $arg_ty),*) -> $ret_ty {
                unsafe { $sys_fn( $($arg),* ) }
            }

            #[inline(never)] // equalize with external calls
            fn asm_fn($(mut $arg: $arg_ty),*) -> $ret_ty {
                use core::arch::asm;
                $(
                    #[$asm_meta]
                    unsafe { $($asm_tt)* }
                )*
            }

            let testvec = <($($arg_ty),*)>::make_testvec($crate::bench::CHECK_ITER_ITEMS);
            let benchvec= <($($arg_ty),*)>::make_testvec($crate::bench::BENCH_ITER_ITEMS);
            let title = stringify!($name);

            // Verify math lines up

            #[cfg($sys_available)]
            for ($($arg),*) in testvec.iter().copied() {
                if $crate::bench::SKIP_SYS_CHECKS.contains(&title) {
                    continue;
                }

                let crate_res = crate_fn($($arg),*);
                let sys_res = sys_fn($($arg),*);
                assert!(
                    $ret_ty::check_eq(crate_res, sys_res),
                    "{title}{:?}: crate: {crate_res:?}, sys: {sys_res:?}",
                    ($($arg),* ,)
                );
            }

            // use a binding to get around nested macro repetition
            let do_asm_check = || {
                for ($($arg),*) in testvec.iter().copied() {
                    // FIXME: these fail for float multiplication
                    // <https://github.com/rust-lang/compiler-builtins/issues/616>
                    if title.contains("mul")
                        // cmp is skipped because builtins do spaceship but assembly does
                        // a single operation.
                        || title.contains("cmp") {
                        continue;
                    }

                    let crate_res = crate_fn($($arg),*);
                    let asm_res = asm_fn($($arg),*);

                    assert!(
                        $ret_ty::check_eq(crate_res, asm_res),
                        "{title}{:?}: crate: {crate_res:?}, asm: {asm_res:?}",
                        ($($arg),* ,)
                    );
                }
            };
            $(
                #[$asm_meta]
                do_asm_check();
            )*

            c.bench_function(&format!("{title} compiler-builtins"), |b| {
                b.iter(|| {
                    for ($($arg),*) in benchvec.iter().copied() {
                        black_box(crate_fn( $(black_box($arg)),* ));
                    }
                })
            });

            #[cfg($sys_available)]
            c.bench_function(&format!("{title} system"), |b| {
                b.iter(|| {
                    for ($($arg),*) in benchvec.iter().copied() {
                        black_box(sys_fn( $(black_box($arg)),* ));
                    }
                })
            });

            // use a binding to get around nested macro repetition
            let mut do_asm_bench = || {
                c.bench_function(&format!(
                    "{title} assembly {} {}", std::env::consts::ARCH, std::env::consts::FAMILY
                ), |b| {
                    b.iter(|| {
                        for ($($arg),*) in benchvec.iter().copied() {
                            black_box(asm_fn( $(black_box($arg)),* ));
                        }
                    })
                });
            };
            $(
                #[$asm_meta]
                do_asm_bench();
            )*
        }
    }};

    (@coalesce $a:ty, $b:ty) => { $a };
    (@coalesce , $b:ty) => { $b };

    // Default to float comparison
    (@eq $f_ty:ty,) => {
        <$f_ty as Float>::eq_repr
    };
    // Use normal eq if the return type is not a float
    (@eq $f_ty:ty, $ret_ty:ty) => {
        |a: $ret_ty, b: $ret_ty| a == b
    };

}

/// A type used as either an input or output to/from a benchmark function.
pub trait BenchType: Sized {
    fn make_testvec(len: u32) -> Vec<Self>;
    fn check_eq(a: Self, b: Self) -> bool;
}

macro_rules! impl_benchtype {
    (float $($f_ty:ty),+) => {$(
        impl BenchType for $f_ty {
            fn make_testvec(len: u32) -> Vec<Self> {
                // refcell because fuzz_* takes a `Fn`
                let ret = RefCell::new(Vec::new());
                crate::fuzz_float(len, |a| ret.borrow_mut().push(a));
                ret.into_inner()
            }

            fn check_eq(a: Self, b: Self) -> bool {
                Float::eq_repr(a, b)
            }
        }

        impl BenchType for ($f_ty, $f_ty) {
            fn make_testvec(len: u32) -> Vec<Self> {
                // refcell because fuzz_* takes a `Fn`
                let ret = RefCell::new(Vec::new());
                crate::fuzz_float_2(len, |a, b| ret.borrow_mut().push((a, b)));
                ret.into_inner()
            }

            fn check_eq(_a: Self, _b: Self) -> bool {
                unimplemented!()
            }
        }
    )*};
    (int $($i_ty:ty),+) => {$(
        impl BenchType for $i_ty {
            fn make_testvec(len: u32) -> Vec<Self> {
                // refcell because fuzz_* takes a `Fn`
                let ret = RefCell::new(Vec::new());
                crate::fuzz(len, |a| ret.borrow_mut().push(a));
                ret.into_inner()
            }

            fn check_eq(a: Self, b: Self) -> bool {
                a == b
            }
        }

        impl BenchType for ($i_ty, $i_ty) {
            fn make_testvec(len: u32) -> Vec<Self> {
                // refcell because fuzz_* takes a `Fn`
                let ret = RefCell::new(Vec::new());
                crate::fuzz_2(len, |a, b| ret.borrow_mut().push((a, b)));
                ret.into_inner()
            }

            fn check_eq(_a: Self, _b: Self) -> bool {
                unimplemented!()
            }
        }
    )*};
}

#[cfg(not(feature = "no-f16-f128"))]
impl_benchtype!(float f16, f128);
impl_benchtype!(float f32, f64);
impl_benchtype!(int i16, i32, i64, i128);
impl_benchtype!(int u16, u32, u64, u128);
