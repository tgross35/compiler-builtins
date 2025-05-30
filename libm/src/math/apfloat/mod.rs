use core::mem::transmute;
use core::ops;

use rustc_apfloat::Float as ApFloat;
#[cfg(f16_enabled)]
use rustc_apfloat::ieee::Half;
#[cfg(f128_enabled)]
use rustc_apfloat::ieee::Quad;
use rustc_apfloat::ieee::{Double, Single};

use super::generic::SqrtHelper;
use crate::support::Float;

macro_rules! fake_float {
    ($FTy:ty, $WrapTy:ident, $ApTy:ty, $one:ident) => {
        #[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
        pub struct $WrapTy($ApTy);

        impl ops::Add for $WrapTy {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self((self.0 + rhs.0).value)
            }
        }

        impl ops::AddAssign for $WrapTy {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl ops::Sub for $WrapTy {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self((self.0 - rhs.0).value)
            }
        }

        impl ops::SubAssign for $WrapTy {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl ops::Mul for $WrapTy {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self((self.0 * rhs.0).value)
            }
        }

        impl ops::MulAssign for $WrapTy {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl ops::Div for $WrapTy {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self((self.0 / rhs.0).value)
            }
        }

        impl ops::Rem for $WrapTy {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self((self.0 % rhs.0).value)
            }
        }

        impl ops::Neg for $WrapTy {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl Float for $WrapTy {
            type Int = <$FTy as Float>::Int;
            type SignedInt = <$FTy as Float>::SignedInt;

            const ZERO: Self = $WrapTy(<$ApTy>::ZERO);
            const INFINITY: Self = $WrapTy(<$ApTy>::INFINITY);
            const NAN: Self = $WrapTy(<$ApTy>::NAN);
            const ONE: Self = $WrapTy($one);

            // TODO these are all garbage
            const NEG_ZERO: Self = $WrapTy(<$ApTy>::ZERO);
            const NEG_ONE: Self = $WrapTy(<$ApTy>::ZERO);
            const NEG_INFINITY: Self = $WrapTy(<$ApTy>::ZERO);
            const NEG_NAN: Self = $WrapTy(<$ApTy>::ZERO);
            const MAX: Self = $WrapTy(<$ApTy>::ZERO);
            const MIN: Self = $WrapTy(<$ApTy>::ZERO);
            const EPSILON: Self = $WrapTy(<$ApTy>::ZERO);
            const PI: Self = $WrapTy(<$ApTy>::ZERO);
            const NEG_PI: Self = $WrapTy(<$ApTy>::ZERO);
            const FRAC_PI_2: Self = $WrapTy(<$ApTy>::ZERO);
            const MIN_POSITIVE_NORMAL: Self = $WrapTy(<$ApTy>::ZERO);

            const BITS: u32 = <$ApTy>::BITS as u32;
            const SIG_BITS: u32 = <$FTy>::SIG_BITS;
            const SIGN_MASK: Self::Int = <$FTy>::SIGN_MASK;
            const SIG_MASK: Self::Int = <$FTy>::SIG_MASK;
            const EXP_MASK: Self::Int = <$FTy>::EXP_MASK;
            const IMPLICIT_BIT: Self::Int = <$FTy>::IMPLICIT_BIT;

            fn to_bits(self) -> Self::Int {
                self.0.to_bits().try_into().unwrap()
            }

            fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            fn is_infinite(self) -> bool {
                self.0.is_infinite()
            }

            fn is_sign_negative(self) -> bool {
                self.0.is_negative()
            }

            fn from_bits(a: Self::Int) -> Self {
                Self(<$ApTy>::from_bits(a.into()))
            }

            fn abs(self) -> Self {
                Self(self.0.abs())
            }

            fn copysign(self, other: Self) -> Self {
                Self(self.0.copy_sign(other.0))
            }

            fn fma(self, y: Self, z: Self) -> Self {
                Self(self.0.mul_add(y.0, z.0).unwrap())
            }

            fn normalize(_significand: Self::Int) -> (i32, Self::Int) {
                todo!()
            }
        }

        impl SqrtHelper for $WrapTy {
            type ISet1 = <$FTy as SqrtHelper>::ISet1;

            type ISet2 = <$FTy as SqrtHelper>::ISet2;

            const FINAL_ROUNDS: u32 = <$FTy>::FINAL_ROUNDS;
        }

        impl $WrapTy {
            pub fn sqrt(self) -> Self {
                crate::math::generic::sqrt(self)
            }

            pub fn fdim(self, other: Self) -> Self {
                crate::math::generic::fdim(self, other)
            }
        }
    };
}

// Curesed, apfloat needs const constructors
#[cfg(f16_enabled)]
const F16_ONE: Half = unsafe { transmute::<[u128; 2], _>([1024, 159766282927565248636202778624]) };
const F32_ONE: Single =
    unsafe { transmute::<[u128; 2], _>([8388608, 113833225510946001209798950912]) };
const F64_ONE: Double =
    unsafe { transmute::<[u128; 2], _>([4503599627370496, 112524264621533123623119749120]) };
#[cfg(f128_enabled)]
const F128_ONE: Quad = unsafe {
    transmute::<[u128; 2], _>([
        5192296858534827628530496329220096,
        112524268163307985775353659392,
    ])
};

#[cfg(f16_enabled)]
fake_float!(f16, ApF16, Half, F16_ONE);
fake_float!(f32, ApF32, Single, F32_ONE);
fake_float!(f64, ApF64, Double, F64_ONE);
#[cfg(f128_enabled)]
fake_float!(f128, ApF128, Quad, F128_ONE);

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    #[cfg(f16_enabled)]
    fn test_f16() {
        let check_sqrt = |x: f16| {
            let lhs = ApF16(Half::from_bits(x.to_bits().into())).sqrt();
            let rhs = ApF16(Half::from_bits(x.sqrt().to_bits().into()));
            if lhs.is_nan() && rhs.is_nan() {
                return;
            }
            assert_eq!(lhs, rhs);
        };

        check_sqrt(0.0);
        check_sqrt(4.0);
        check_sqrt(1234.23);
        check_sqrt(-2.0);
        check_sqrt(f16::MAX);
    }

    #[test]
    fn test_f32() {
        let check_sqrt = |x: f32| {
            let lhs = ApF32(Single::from_bits(x.to_bits().into())).sqrt();
            let rhs = ApF32(Single::from_bits(x.sqrt().to_bits().into()));
            if lhs.is_nan() && rhs.is_nan() {
                return;
            }
            assert_eq!(lhs, rhs);
        };

        check_sqrt(0.0);
        check_sqrt(4.0);
        check_sqrt(123421.23423);
        check_sqrt(-2.0);
        check_sqrt(f32::MAX);
    }

    #[test]
    fn test_f64() {
        let check_sqrt = |x: f64| {
            let lhs = ApF64(Double::from_bits(x.to_bits().into())).sqrt();
            let rhs = ApF64(Double::from_bits(x.sqrt().to_bits().into()));
            if lhs.is_nan() && rhs.is_nan() {
                return;
            }
            assert_eq!(lhs, rhs);
        };

        check_sqrt(0.0);
        check_sqrt(4.0);
        check_sqrt(12334421.23234423);
        check_sqrt(-2.0);
        check_sqrt(f64::MAX);
    }

    #[test]
    #[cfg(f128_enabled)]
    fn test_f128() {
        let check_sqrt = |x: f128| {
            let lhs = ApF128(Quad::from_bits(x.to_bits().into())).sqrt();
            let rhs = ApF128(Quad::from_bits(x.sqrt().to_bits().into()));
            if lhs.is_nan() && rhs.is_nan() {
                return;
            }
            assert_eq!(lhs, rhs);
        };

        check_sqrt(0.0);
        check_sqrt(4.0);
        check_sqrt(12342110234.23123432423);
        check_sqrt(-2.0);
        check_sqrt(f128::MAX);
    }
}
