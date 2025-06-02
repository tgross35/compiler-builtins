/* SPDX-License-Identifier: MIT OR Apache-2.0 */
//! IEEE 754-2019 `maximumNumber`.
//!
//! Per the spec, returns:
//! - `x` if `x > y`
//! - `y` if `y > x`
//! - +0.0 if x and y are zero with opposite signs
//! - Either `x` or `y` if `x == y` and the signs are the same
//! - Non-NaN if one operand is NaN
//! - qNaN if both operands are NaNx
//!
//! Excluded from our implementation is sNaN handling.

use crate::support::Float;

#[inline]
pub fn fmaximum_num<F: Float>(x: F, y: F) -> F {
    let y_ = y.is_nan();
    let f = x > y;
    let s = y > x;
    let x_ = x.is_nan();
    let p = x.is_sign_positive();

    if (!f && !y_ && !p) || (!f && !y_ && x_) || (!f && !y_ && s) {
        y
    } else {
        x
    }

    // if f || y_ {
    //     x
    // } else if s || x_ {
    //     y
    // } else if p {
    //     x
    // } else {
    //     y
    // }
    // if x > y || y.is_nan() {
    //     x
    // } else if y > x || x.is_nan() {
    //     y
    // } else if x.is_sign_positive() {
    //     x
    // } else {
    //     y
    // }
}
