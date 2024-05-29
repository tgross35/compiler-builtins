use core::ops::Neg;

use crate::int::{CastFrom, CastInto, Int, MinInt};

use super::Float;

/// Conversions from integers to floats.
///
/// These are hand-optimized bit twiddling code,
/// which unfortunately isn't the easiest kind of code to read.
///
/// The algorithm is explained here: <https://blog.m-ou.se/floats/>
fn int_to_float<I, F>(i: I) -> F
where
    F: Float,
    I: Int<UnsignedInt: Int>,
    I::UnsignedInt: CastInto<F::Int>,
    F::Int: CastFrom<u32>,
    F::Int: CastFrom<I>,
    F::Int: From<bool>,
    F::Int: CastInto<I::UnsignedInt>,
{
    if i == I::ZERO {
        return F::ZERO;
    }

    let sign_bit: F::Int = if I::SIGNED {
        F::Int::cast_from(i >> (I::BITS - 1)) << (F::BITS - 1)
    } else {
        // Never used
        F::Int::ZERO
    };

    let i = i.unsigned_abs();
    let n = i.leading_zeros();

    // Calculate the exponent from the integer's significant digits
    let e = F::Int::cast_from(I::BITS + F::EXPONENT_BIAS - 2 - n);

    // The mantissa of `i`, still in `I`'s form (left shifted so the first bit is nonzero)
    let i_m = i.wrapping_shl(n);

    let m: F::Int = if F::BITS > I::BITS {
        F::Int::cast_from(i) << (F::SIGNIFICAND_BITS - I::BITS + 1 + n)
    } else {
        // Shift the integer into the float's mantissa bits. Keep the lowest
        // exponent bit intact.
        let m_base = F::Int::cast_from(i_m >> ((I::BITS - F::BITS) + F::EXPONENT_BITS));

        // Squash dropped bits into a single `F::Int` for comparison. The result will have
        // no meaning outside of indicating rounding.
        let dropped_bits: F::Int = if F::BITS == I::BITS {
            // Simple case

            // Only the lowest `F::EXPONENT_BITS` bits will be truncated. Shift them
            // to the highest position
            (i_m << (F::SIGNIFICAND_BITS + 1)).cast()
        } else if F::BITS * 2 == I::BITS {
            // Specialized case where the float is half the integer size

            // The entire lower half of `i` will be truncated (masked portion), plus the
            // next `EXPONENT_BITS` bits.
            let mask: I::UnsignedInt = (F::Int::MAX >> (F::Int::BITS / 2)).cast();
            (i_m >> F::EXPONENT_BITS | i_m & mask).cast()
        } else {
            // Generic case

            // Within the upper `F::BITS`, everything except for the signifcand
            // gets truncated
            let d1: F::Int = (i_m >> (I::BITS - F::BITS - F::SIGNIFICAND_BITS - 1)).cast();

            // The entire rest of `i_m` gets truncated. Zero the upper `F::BITS` then just
            // check if it is nonzero.
            let d2: F::Int = (i_m << F::BITS >> F::BITS != I::UnsignedInt::ZERO).into();

            d1 | d2
        };

        // Branchlessly extract a `1` if rounding up should happen
        let m_adj = (dropped_bits - (dropped_bits >> (F::BITS - 1) & !m_base)) >> (F::BITS - 1);

        // Add one when we need to round up. Break ties to even.
        m_base + m_adj
    };

    // + not |, so the mantissa can overflow into the exponent.
    let urepr = (e << F::SIGNIFICAND_BITS) + m;
    let repr: F::Int = if I::SIGNED { urepr | sign_bit } else { urepr };

    F::from_repr(repr)
}

// Conversions from unsigned integers to floats.
intrinsics! {
    #[arm_aeabi_alias = __aeabi_ui2f]
    pub extern "C" fn __floatunsisf(i: u32) -> f32 {
        int_to_float(i)
    }

    #[arm_aeabi_alias = __aeabi_ui2d]
    pub extern "C" fn __floatunsidf(i: u32) -> f64 {
        int_to_float(i)
    }

    #[arm_aeabi_alias = __aeabi_ul2f]
    pub extern "C" fn __floatundisf(i: u64) -> f32 {
        int_to_float(i)
    }

    #[arm_aeabi_alias = __aeabi_ul2d]
    pub extern "C" fn __floatundidf(i: u64) -> f64 {
        int_to_float(i)
    }

    #[cfg_attr(target_os = "uefi", unadjusted_on_win64)]
    pub extern "C" fn __floatuntisf(i: u128) -> f32 {
        int_to_float(i)
    }

    #[cfg_attr(target_os = "uefi", unadjusted_on_win64)]
    pub extern "C" fn __floatuntidf(i: u128) -> f64 {
        int_to_float(i)
    }

    #[ppc_alias = __floatunsikf]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __floatunsitf(i: u32) -> f128 {
        int_to_float(i)
    }

    #[ppc_alias = __floatundikf]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __floatunditf(i: u64) -> f128 {
        int_to_float(i)
    }

    #[ppc_alias = __floatuntikf]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __floatuntitf(i: u128) -> f128 {
        int_to_float(i)
    }
}

// Conversions from signed integers to floats.
intrinsics! {
    #[arm_aeabi_alias = __aeabi_i2f]
    pub extern "C" fn __floatsisf(i: i32) -> f32 {
        int_to_float(i)
    }

    #[arm_aeabi_alias = __aeabi_i2d]
    pub extern "C" fn __floatsidf(i: i32) -> f64 {
        int_to_float(i)
    }

    #[arm_aeabi_alias = __aeabi_l2f]
    pub extern "C" fn __floatdisf(i: i64) -> f32 {
        int_to_float(i)
    }

    #[arm_aeabi_alias = __aeabi_l2d]
    pub extern "C" fn __floatdidf(i: i64) -> f64 {
        int_to_float(i)
    }

    #[cfg_attr(target_os = "uefi", unadjusted_on_win64)]
    pub extern "C" fn __floattisf(i: i128) -> f32 {
        int_to_float(i)
    }

    #[cfg_attr(target_os = "uefi", unadjusted_on_win64)]
    pub extern "C" fn __floattidf(i: i128) -> f64 {
        int_to_float(i)
    }

    #[ppc_alias = __floatsikf]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __floatsitf(i: i32) -> f128 {
        int_to_float(i)
    }

    #[ppc_alias = __floatdikf]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __floatditf(i: i64) -> f128 {
        int_to_float(i)
    }

    #[ppc_alias = __floattikf]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __floattitf(i: i128) -> f128 {
        int_to_float(i)
    }
}

/// Generic float to unsigned int conversions.
fn float_to_unsigned_int<F, U>(f: F) -> U
where
    F: Float,
    U: Int<UnsignedInt = U>,
    F::Int: CastInto<U>,
    F::Int: CastFrom<u32>,
    F::Int: CastInto<U::UnsignedInt>,
    u32: CastFrom<F::Int>,
{
    float_to_int_inner::<F, U, _, _>(f.repr(), |i: U| i, || U::MAX)
}

/// Generic float to signed int conversions.
fn float_to_signed_int<F, I>(f: F) -> I
where
    F: Float,
    I: Int + Neg<Output = I>,
    I::UnsignedInt: Int,
    F::Int: CastInto<I::UnsignedInt>,
    F::Int: CastFrom<u32>,
    u32: CastFrom<F::Int>,
{
    float_to_int_inner::<F, I, _, _>(
        f.repr() & !F::SIGN_MASK,
        |i: I| if f.is_sign_negative() { -i } else { i },
        || if f.is_sign_negative() { I::MIN } else { I::MAX },
    )
}

/// Float to int conversions, generic for both signed and unsigned.
///
/// Parameters:
/// - `fbits`: `abg(f)` bitcasted to an integer.
/// - `map_inbounds`: apply this transformation to integers that are within range (add the sign
///    back).
/// - `out_of_bounds`: return value when out of range for `I`.
fn float_to_int_inner<F, I, FnFoo, FnOob>(
    fbits: F::Int,
    map_inbounds: FnFoo,
    out_of_bounds: FnOob,
) -> I
where
    F: Float,
    I: Int,
    FnFoo: FnOnce(I) -> I,
    FnOob: FnOnce() -> I,
    I::UnsignedInt: Int,
    F::Int: CastInto<I::UnsignedInt>,
    F::Int: CastFrom<u32>,
    u32: CastFrom<F::Int>,
{
    let int_max_exp = F::EXPONENT_BIAS + I::MAX.ilog2() + 1;
    let foobar = F::EXPONENT_BIAS + I::UnsignedInt::BITS - 1;

    if fbits < F::ONE.repr() {
        // < 0 gets rounded to 0
        I::ZERO
    } else if fbits < F::Int::cast_from(int_max_exp) << F::SIGNIFICAND_BITS {
        // >= 1, < integer max
        let m_base = if I::UnsignedInt::BITS >= F::Int::BITS {
            I::UnsignedInt::cast_from(fbits) << (I::BITS - F::SIGNIFICAND_BITS - 1)
        } else {
            I::UnsignedInt::cast_from(fbits >> (F::SIGNIFICAND_BITS - I::BITS + 1))
        };

        // Set the implicit 1-bit.
        let m: I::UnsignedInt = I::UnsignedInt::ONE << (I::BITS - 1) | m_base;

        // Shift based on the exponent and bias.
        let s: u32 = (foobar) - u32::cast_from(fbits >> F::SIGNIFICAND_BITS);

        let unsigned = m >> s;
        map_inbounds(I::from_unsigned(unsigned))
    } else if fbits <= F::EXPONENT_MASK {
        // >= max (incl. inf)
        out_of_bounds()
    } else {
        I::ZERO
    }
}

// Conversions from floats to unsigned integers.
intrinsics! {
    #[arm_aeabi_alias = __aeabi_f2uiz]
    pub extern "C" fn __fixunssfsi(f: f32) -> u32 {
        float_to_unsigned_int(f)
    }

    #[arm_aeabi_alias = __aeabi_f2ulz]
    pub extern "C" fn __fixunssfdi(f: f32) -> u64 {
        float_to_unsigned_int(f)
    }

    #[win64_128bit_abi_hack]
    pub extern "C" fn __fixunssfti(f: f32) -> u128 {
        float_to_unsigned_int(f)
    }

    #[arm_aeabi_alias = __aeabi_d2uiz]
    pub extern "C" fn __fixunsdfsi(f: f64) -> u32 {
        float_to_unsigned_int(f)
    }

    #[arm_aeabi_alias = __aeabi_d2ulz]
    pub extern "C" fn __fixunsdfdi(f: f64) -> u64 {
        float_to_unsigned_int(f)
    }

    #[win64_128bit_abi_hack]
    pub extern "C" fn __fixunsdfti(f: f64) -> u128 {
        float_to_unsigned_int(f)
    }

    #[ppc_alias = __fixunskfsi]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __fixunstfsi(f: f128) -> u32 {
        float_to_unsigned_int(f)
    }

    #[ppc_alias = __fixunskfdi]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __fixunstfdi(f: f128) -> u64 {
        float_to_unsigned_int(f)
    }

    #[ppc_alias = __fixunskfti]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __fixunstfti(f: f128) -> u128 {
        float_to_unsigned_int(f)
    }
}

// Conversions from floats to signed integers.
intrinsics! {
    #[arm_aeabi_alias = __aeabi_f2iz]
    pub extern "C" fn __fixsfsi(f: f32) -> i32 {
        float_to_signed_int(f)
    }

    #[arm_aeabi_alias = __aeabi_f2lz]
    pub extern "C" fn __fixsfdi(f: f32) -> i64 {
        float_to_signed_int(f)
    }

    #[win64_128bit_abi_hack]
    pub extern "C" fn __fixsfti(f: f32) -> i128 {
        float_to_signed_int(f)
    }

    #[arm_aeabi_alias = __aeabi_d2iz]
    pub extern "C" fn __fixdfsi(f: f64) -> i32 {
        float_to_signed_int(f)
    }

    #[arm_aeabi_alias = __aeabi_d2lz]
    pub extern "C" fn __fixdfdi(f: f64) -> i64 {
        float_to_signed_int(f)
    }

    #[win64_128bit_abi_hack]
    pub extern "C" fn __fixdfti(f: f64) -> i128 {
        float_to_signed_int(f)
    }

    #[ppc_alias = __fixkfsi]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __fixtfsi(f: f128) -> i32 {
        float_to_signed_int(f)
    }

    #[ppc_alias = __fixkfdi]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __fixtfdi(f: f128) -> i64 {
        float_to_signed_int(f)
    }

    #[ppc_alias = __fixkfti]
    #[cfg(not(feature = "no-f16-f128"))]
    pub extern "C" fn __fixtfti(f: f128) -> i128 {
        float_to_signed_int(f)
    }
}
