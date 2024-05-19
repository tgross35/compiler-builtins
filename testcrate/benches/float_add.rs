#![feature(f128)]

use compiler_builtins::float::add;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: add_f32,
    sig: (a: f32, b: f32) -> f32,
    crate_fn: add::__addsf3,
    sys_fn: __addsf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "addss xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fadd    s0, s0, s1",
            "ret",
        );
    ],
}

float_bench! {
    name: add_f64,
    sig: (a: f64, b: f64) -> f64,
    crate_fn: add::__adddf3,
    sys_fn: __adddf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "addsd xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fadd    d0, d0, d1",
            "ret",
        );
    ],
}

float_bench! {
    name: add_f128,
    sig: (a: f128, b: f128) -> f128,
    crate_fn: add::__addtf3,
    sys_fn: __addtf3,
    sys_available: not(feature = "no-sys-f128"),
    asm: []
}

criterion_group!(float_add, add_f32, add_f64, add_f128);
criterion_main!(float_add);
