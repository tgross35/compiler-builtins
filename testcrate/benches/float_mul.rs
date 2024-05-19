#![feature(f128)]

use compiler_builtins::float::mul;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: mul_f32,
    sig: (a: f32, b: f32) -> f32,
    crate_fn: mul::__mulsf3,
    sys_fn: __mulsf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "mulss xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
           "fmul    s0, s0, s1",
           "ret",
        );
    ],
}

float_bench! {
    name: mul_f64,
    sig: (a: f64, b: f64) -> f64,
    crate_fn: mul::__muldf3,
    sys_fn: __muldf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "mulsd xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
           "fmul    d0, d0, d1",
           "ret",
        );
    ],
}

float_bench! {
    name: mul_f128,
    sig: (a: f128, b: f128) -> f128,
    crate_fn: mul::__multf3,
    sys_fn: __multf3,
    sys_available: not(feature = "no-sys-f128"),
    asm: []
}

criterion_group!(float_mul, mul_f32, mul_f64, mul_f128);
criterion_main!(float_mul);
