#![feature(f128)]

use compiler_builtins::float::sub;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: sub_f32,
    sig: (a: f32, b: f32) -> f32,
    crate_fn: sub::__subsf3,
    sys_fn: __subsf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "subss xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
           "fsub    s0, s0, s1",
           "ret",
        );
    ],
}

float_bench! {
    name: sub_f64,
    sig: (a: f64, b: f64) -> f64,
    crate_fn: sub::__subdf3,
    sys_fn: __subdf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "subsd xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
           "fsub    d0, d0, d1",
           "ret",
        );
    ],
}

float_bench! {
    name: sub_f128,
    sig: (a: f128, b: f128) -> f128,
    crate_fn: sub::__subtf3,
    sys_fn: __subtf3,
    sys_available: not(feature = "no-sys-f128"),
    asm: []
}

criterion_group!(float_sub, sub_f32, sub_f64, sub_f128);
criterion_main!(float_sub);
