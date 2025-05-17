//! libm in pure Rust
#![no_std]
// #![feature(cfg_target_has_reliable_f16_f128)]
#![cfg_attr(intrinsics_enabled, feature(core_intrinsics))]
#![cfg_attr(
    all(intrinsics_enabled, target_family = "wasm"),
    feature(wasm_numeric_instr)
)]
#![cfg_attr(
    any(intrinsics_enabled, feature = "unstable-float"),
    allow(internal_features)
)]
#![cfg_attr(feature = "unstable-float", feature(cfg_targest_has_reliable_f16_f128))]
#![cfg_attr(all(feature = "unstable-float", target_has_reliable_f16), feature(f16))]
#![cfg_attr(
    all(feature = "unstable-float", target_has_reliable_f128),
    feature(f128)
)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::deprecated_cfg_attr)]
#![allow(clippy::eq_op)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::float_cmp)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::mixed_case_hex_literals)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_return)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::zero_divided_by_zero)]
#![forbid(unsafe_op_in_unsafe_fn)]

mod libm_helper;
mod math;

use core::{f32, f64};

pub use libm_helper::*;

pub use self::math::*;
