#![feature(f128)]

use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

use compiler_builtins::float::cmp;

float_bench! {
    name: cmp_f32_gt,
    sig: (a: f32, b: f32) -> i32,
    crate_fn: cmp::__gtsf2,
    sys_fn: __gtsf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "xor     eax, eax",
            "ucomiss xmm0, xmm1",
            "seta    al",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcmp    s0, s1",
            "cset    w0, gt",
            "ret",
        );
    ],
}

float_bench! {
    name: cmp_f32_unord,
    sig: (a: f32, b: f32) -> i32,
    crate_fn: cmp::__unordsf2,
    sys_fn: __unordsf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "cmpneqss xmm0, xmm1",
            "movd     eax, xmm0",
            "and      eax, 1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcmp    s0, s1",
            "cset    w0, eq",
            "ret",
        );
    ],
}

float_bench! {
    name: cmp_f64_gt,
    sig: (a: f64, b: f64) -> i32,
    crate_fn: cmp::__gtdf2,
    sys_fn: __gtdf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "xor     eax, eax",
            "ucomisd xmm0, xmm1",
            "seta    al",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcmp    d0, d1",
            "cset    w0, gt",
            "ret",
        );
    ],
}

float_bench! {
    name: cmp_f64_unord,
    sig: (a: f64, b: f64) -> i32,
    crate_fn: cmp::__unorddf2,
    sys_fn: __unorddf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")]
        asm!(
            "cmpeqsd xmm0, xmm1",
            "movq    rax, xmm0",
            "and     eax, 1",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "fcmp    d0, d1",
            "cset    w0, eq",
            "ret",
        );
    ],
}

float_bench! {
    name: cmp_f128_gt,
    sig: (a: f128, b: f128) -> i32,
    crate_fn: cmp::__gttf2,
    sys_fn: __gttf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: []
}

float_bench! {
    name: cmp_f128_unord,
    sig: (a: f128, b: f128) -> i32,
    crate_fn: cmp::__unordtf2,
    sys_fn: __unordtf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: []
}

criterion_group!(
    float_cmp,
    cmp_f32_gt,
    cmp_f32_unord,
    cmp_f64_gt,
    cmp_f64_unord,
    cmp_f128_gt,
    cmp_f128_unord
);
criterion_main!(float_cmp);
