use super::lgammaf64_r;

/// The natural logarithm of the
/// [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64).
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn lgammaf64(x: f64) -> f64 {
    lgammaf64_r(x).0
}
