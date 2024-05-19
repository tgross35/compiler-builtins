#![feature(f128)]
#![allow(improper_ctypes)]

use compiler_builtins::float::conv;
use criterion::{criterion_group, criterion_main, Criterion};
use testcrate::float_bench;

/* unsigned int -> float */

float_bench! {
    name: conv_u32_f32,
    sig: (i: u32) -> f32,
    crate_fn: conv::__floatunsisf,
    sys_fn: __floatunsisf,
    sys_available: all(),
    asm: [
        #[cfg(all(target_arch = "x86_64", not(target_family = "windows")))]
        asm!(
            "mov         eax, edi",
            "cvtsi2ss    xmm0, rax",
            "ret",
        );

        #[cfg(all(target_arch = "x86_64", target_family = "windows"))]
        asm!(
            "mov         eax, ecx",
            "cvtsi2ss    xmm0, rax",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "ucvtf   s0, w0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_u32_f64,
    sig: (i: u32) -> f64,
    crate_fn: conv::__floatunsidf,
    sys_fn: __floatunsidf,
    sys_available: all(),
    asm: [
        #[cfg(all(target_arch = "x86_64", not(target_family = "windows")))]
        asm!(
            "mov         eax, edi",
            "cvtsi2sd    xmm0, rax",
            "ret",
        );

        #[cfg(all(target_arch = "x86_64", target_family = "windows"))]
        asm!(
            "mov         eax, ecx",
            "cvtsi2sd    xmm0, rax",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "ucvtf   d0, w0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_u64_f32,
    sig: (i: u64) -> f32,
    crate_fn: conv::__floatundisf,
    sys_fn: __floatundisf,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
            "ucvtf   s0, x0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_u64_f64,
    sig: (i: u64) -> f64,
    crate_fn: conv::__floatundidf,
    sys_fn: __floatundidf,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
            "ucvtf   d0, x0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_u128_f32,
    sig: (i: u128) -> f32,
    crate_fn: conv::__floatuntisf,
    sys_fn: __floatuntisf,
    sys_available: all(),
    asm: []
}

float_bench! {
    name: conv_u128_f64,
    sig: (i: u128) -> f64,
    crate_fn: conv::__floatuntidf,
    sys_fn: __floatuntidf,
    sys_available: all(),
    asm: []
}

/* signed int -> float */

float_bench! {
    name: conv_i32_f32,
    sig: (i: i32) -> f32,
    crate_fn: conv::__floatsisf,
    sys_fn: __floatsisf,
    sys_available: all(),
    asm: [
        #[cfg(all(target_arch = "x86_64", not(target_family = "windows")))]
        asm!(
            "cvtsi2ss    xmm0, edi",
            "ret",
        );

        #[cfg(all(target_arch = "x86_64", target_family = "windows"))]
        asm!(
            "cvtsi2ss    xmm0, ecx",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "scvtf   s0, w0",
            "ret",

        );
    ],
}

float_bench! {
    name: conv_i32_f64,
    sig: (i: i32) -> f64,
    crate_fn: conv::__floatsidf,
    sys_fn: __floatsidf,
    sys_available: all(),
    asm: [
        #[cfg(all(target_arch = "x86_64", not(target_family = "windows")))]
        asm!(
            "cvtsi2sd    xmm0, edi",
            "ret",
        );

        #[cfg(all(target_arch = "x86_64", target_family = "windows"))]
        asm!(
            "cvtsi2sd    xmm0, ecx",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "scvtf   d0, w0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_i64_f32,
    sig: (i: i64) -> f32,
    crate_fn: conv::__floatdisf,
    sys_fn: __floatdisf,
    sys_available: all(),
    asm: [
        #[cfg(all(target_arch = "x86_64", not(target_family = "windows")))]
        asm!(
            "cvtsi2ss    xmm0, rdi",
            "ret",
        );

        #[cfg(all(target_arch = "x86_64", target_family = "windows"))]
        asm!(
            "cvtsi2ss    xmm0, rcx",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "scvtf   s0, x0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_i64_f64,
    sig: (i: i64) -> f64,
    crate_fn: conv::__floatdidf,
    sys_fn: __floatdidf,
    sys_available: all(),
    asm: [
        #[cfg(all(target_arch = "x86_64", not(target_family = "windows")))]
        asm!(
            "cvtsi2sd    xmm0, rdi",
            "ret",
        );

        #[cfg(all(target_arch = "x86_64", target_family = "windows"))]
        asm!(
            "cvtsi2sd    xmm0, rcx",
            "ret",
        );

        #[cfg(target_arch = "aarch64")]
        asm!(
            "scvtf   d0, x0",
            "ret",
        );
    ],
}

float_bench! {
    name: conv_i128_f32,
    sig: (i: i128) -> f32,
    crate_fn: conv::__floattisf,
    sys_fn: __floattisf,
    sys_available: all(),
    asm: []
}

float_bench! {
    name: conv_i128_f64,
    sig: (i: i128) -> f64,
    crate_fn: conv::__floattidf,
    sys_fn: __floattidf,
    sys_available: all(),
    asm: []
}

/* float -> unsigned int */

float_bench! {
    name: conv_f32_u32,
    sig: (f: f32) -> u32,
    crate_fn: conv::__fixunssfsi,
    sys_fn: __fixunssfsi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzu  w0, s0",
           "ret",
        );
    ],
}
float_bench! {
    name: conv_f32_u64,
    sig: (f: f32) -> u64,
    crate_fn: conv::__fixunssfdi,
    sys_fn: __fixunssfdi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzu  x0, s0",
           "ret",
        );
    ],
}

float_bench! {
    name: conv_f32_u128,
    sig: (f: f32) -> u128,
    crate_fn: conv::__fixunssfti,
    sys_fn: __fixunssfti,
    sys_available: all(),
    asm: []
}

float_bench! {
    name: conv_f64_u32,
    sig: (f: f64) -> u32,
    crate_fn: conv::__fixunsdfsi,
    sys_fn: __fixunsdfsi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzu  w0, d0",
           "ret",
        );
    ],
}

float_bench! {
    name: conv_f64_u64,
    sig: (f: f64) -> u64,
    crate_fn: conv::__fixunsdfdi,
    sys_fn: __fixunsdfdi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzu  x0, d0",
           "ret",
        );
    ],
}

float_bench! {
    name: conv_f64_u128,
    sig: (f: f64) -> u128,
    crate_fn: conv::__fixunsdfti,
    sys_fn: __fixunsdfti,
    sys_available: all(),
    asm: []
}

/* float -> signed int */

float_bench! {
    name: conv_f32_i32,
    sig: (f: f32) -> i32,
    crate_fn: conv::__fixsfsi,
    sys_fn: __fixsfsi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzs  w0, s0",
           "ret",
        );
    ],
}
float_bench! {
    name: conv_f32_i64,
    sig: (f: f32) -> i64,
    crate_fn: conv::__fixsfdi,
    sys_fn: __fixsfdi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzs  x0, s0",
           "ret",
        );
    ],
}

float_bench! {
    name: conv_f32_i128,
    sig: (f: f32) -> i128,
    crate_fn: conv::__fixsfti,
    sys_fn: __fixsfti,
    sys_available: all(),
    asm: []
}

float_bench! {
    name: conv_f64_i32,
    sig: (f: f64) -> i32,
    crate_fn: conv::__fixdfsi,
    sys_fn: __fixdfsi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzs  w0, d0",
           "ret",
        );
    ],
}

float_bench! {
    name: conv_f64_i64,
    sig: (f: f64) -> i64,
    crate_fn: conv::__fixdfdi,
    sys_fn: __fixdfdi,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "aarch64")]
        asm!(
           "fcvtzs  x0, d0",
           "ret",
        );
    ],
}

float_bench! {
    name: conv_f64_i128,
    sig: (f: f64) -> i128,
    crate_fn: conv::__fixdfti,
    sys_fn: __fixdfti,
    sys_available: all(),
    asm: []
}

criterion_group!(
    float_conv,
    conv_u32_f32,
    conv_u32_f64,
    conv_u64_f32,
    conv_u64_f64,
    conv_u128_f32,
    conv_u128_f64,
    conv_i32_f32,
    conv_i32_f64,
    conv_i64_f32,
    conv_i64_f64,
    conv_i128_f32,
    conv_i128_f64,
    conv_f32_u32,
    conv_f32_u64,
    conv_f32_u128,
    conv_f32_i32,
    conv_f32_i64,
    conv_f32_i128,
    conv_f64_u32,
    conv_f64_u64,
    conv_f64_u128,
    conv_f64_i32,
    conv_f64_i64,
    conv_f64_i128,
);
criterion_main!(float_conv);
