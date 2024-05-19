#![feature(f128)]
#![feature(f16)]

use compiler_builtins::float::trunc;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: trunc_f32_f16,
    sig: (f: f32) -> f16,
    crate_fn: trunc::__truncsfhf2,
    sys_fn: __truncsfhf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcvt    h0, s0",
            "ret",
        );
    ],
}

float_bench! {
    name: trunc_f64_f16,
    sig: (f: f64) -> f16,
    crate_fn: trunc::__truncdfhf2,
    sys_fn: __truncdfhf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcvt    h0, d0",
            "ret",
        );
    ],
}

float_bench! {
    name: trunc_f64_f32,
    sig: (f: f64) -> f32,
    crate_fn: trunc::__truncdfsf2,
    sys_fn: __truncdfsf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "cvtsd2ss        xmm0, xmm0",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcvt    s0, d0",
            "ret",
        );
    ],
}

float_bench! {
    name: trunc_f128_f16,
    sig: (f: f128) -> f16,
    crate_fn: trunc::__trunctfhf2,
    sys_fn: __trunctfhf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

float_bench! {
    name: trunc_f128_f32,
    sig: (f: f128) -> f32,
    crate_fn: trunc::__trunctfsf2,
    sys_fn: __trunctfsf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

float_bench! {
    name: trunc_f128_f64,
    sig: (f: f128) -> f64,
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
