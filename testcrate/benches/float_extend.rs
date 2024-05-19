#![feature(f128)]
#![feature(f16)]

use compiler_builtins::float::extend;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

float_bench! {
    name: extend_f16_f32,
    sig: (f: f16) -> f32,
    crate_fn: extend::__extendhfsf2,
    sys_fn: __extendhfsf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcvt    s0, h0",
            "ret",
        );
    ],
}

float_bench! {
    name: extend_f16_f128,
    sig: (f: f16) -> f128,
    crate_fn: extend::__extendhftf2,
    sys_fn: __extendhftf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

float_bench! {
    name: extend_f32_f64,
    sig: (f: f32) -> f64,
    crate_fn: extend::__extendsfdf2,
    sys_fn: __extendsfdf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcvt    d0, s0",
            "ret",
        );
    ],
}

float_bench! {
    name: extend_f32_f128,
    sig: (f: f32) -> f128,
    crate_fn: extend::__extendsftf2,
    sys_fn: __extendsftf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

float_bench! {
    name: extend_f64_f128,
    sig: (f: f64) -> f128,
    crate_fn: extend::__extenddftf2,
    sys_fn: __extenddftf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: [],
}

criterion_group!(
    float_extend,
    extend_f16_f32,
    extend_f16_f128,
    extend_f32_f64,
    extend_f32_f128,
    extend_f64_f128,
);
criterion_main!(float_extend);
