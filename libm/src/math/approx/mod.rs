//! Approximate implementations.
//!
//! These functions may be smaller or faster than those in the main `math` module, but will
//! not be as accurate.

mod cbrtf64;
mod lgammaf64;
mod lgammaf64_r;

pub use cbrtf64::cbrtf64;
pub use lgammaf64::lgammaf64;
pub use lgammaf64_r::lgammaf64_r;
