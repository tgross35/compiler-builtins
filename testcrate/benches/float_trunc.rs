#![feature(f128)]
#![feature(f16)]

use compiler_builtins::float::trunc;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: trunc_f32_f16,
    sig: (a: f32) -> f16,
    crate_fn: trunc::__truncsfhf2,
    sys_fn: __truncsfhf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")] {
            // FIXME(f16_f128): use `f16` assembly once support is added (rust-lang/rust/#116909)
            let ret = core::mem::MaybeUninit::uninit();
            asm!(
                "fcvt    h0, {a:s}",
                a = in(vreg) a,
                // ret = lateout(vreg) ret,
                options(nomem, nostack),
            );

            ret.assume_init()
        };
    ],
}

float_bench! {
    name: trunc_f64_f16,
    sig: (a: f64) -> f16,
    crate_fn: trunc::__truncdfhf2,
    sys_fn: __truncdfhf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [
        #[cfg(target_arch = "aarch64")] {
            // FIXME(f16_f128): use `f16` assembly once support is added (rust-lang/rust/#116909)
            let ret = core::mem::MaybeUninit::uninit();
            asm!(
                "fcvt    h0, {a:d}",
                a = in(vreg) a,
                // ret = lateout(vreg) ret,
                options(nomem, nostack),
            );

            ret.assume_init()
        };
    ],
}

float_bench! {
    name: trunc_f64_f32,
    sig: (a: f64) -> f32,
    crate_fn: trunc::__truncdfsf2,
    sys_fn: __truncdfsf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")] {
            let ret: f32;
            asm!(
                "cvtsd2ss {ret}, {a}",
                a = in(xmm_reg) a,
                ret = lateout(xmm_reg) ret,
                options(nomem, nostack),
            );

            ret
        };

        #[cfg(target_arch = "aarch64")] {
            let ret: f32;
            asm!(
                "fcvt    {ret:s}, {a:d}",
                a = in(vreg) a,
                ret = lateout(vreg) ret,
                options(nomem, nostack),
            );

            ret
        };
    ],
}

float_bench! {
    name: trunc_f128_f16,
    sig: (a: f128) -> f16,
    crate_fn: trunc::__trunctfhf2,
    sys_fn: __trunctfhf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

float_bench! {
    name: trunc_f128_f32,
    sig: (a: f128) -> f32,
    crate_fn: trunc::__trunctfsf2,
    sys_fn: __trunctfsf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

float_bench! {
    name: trunc_f128_f64,
    sig: (a: f128) -> f64,
    crate_fn: trunc::__trunctfdf2,
    sys_fn: __trunctfdf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

criterion_group!(
    float_trunc,
    trunc_f32_f16,
    trunc_f64_f16,
    trunc_f64_f32,
    trunc_f128_f16,
    trunc_f128_f32,
    trunc_f128_f64,
);
criterion_main!(float_trunc);
