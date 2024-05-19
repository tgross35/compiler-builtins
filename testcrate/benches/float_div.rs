#![feature(f128)]

use compiler_builtins::float::div;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: div_f32,
    sig: (a: f32, b: f32) -> f32,
    crate_fn: div::__divsf3,
    sys_fn: __divsf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "divss xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
           "fdiv    s0, s0, s1",
           "ret",
        );
    ],
}

float_bench! {
    name: div_f64,
    sig: (a: f64, b: f64) -> f64,
    crate_fn: div::__divdf3,
    sys_fn: __divdf3,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "divsd xmm0, xmm1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
           "fdiv    d0, d0, d1",
           "ret",
        );
    ],
}

criterion_group!(float_div, div_f32, div_f64);
criterion_main!(float_div);
