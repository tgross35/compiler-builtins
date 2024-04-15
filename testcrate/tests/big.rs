use compiler_builtins::int::{i256, u256, HInt};

const LOHI_SPLIT: u128 = 0xaaaaaaaaaaaaaaaaffffffffffffffff;

#[test]
fn widen_u128() {
    assert_eq!(u128::MAX.widen(), u256([u64::MAX, u64::MAX, 0, 0]));
    assert_eq!(
        LOHI_SPLIT.widen(),
        u256([u64::MAX, 0xaaaaaaaaaaaaaaaa, 0, 0])
    );
}

#[test]
fn widen_i128() {
    assert_eq!((-1i128).widen(), u256::MAX.signed());
    assert_eq!(
        (LOHI_SPLIT as i128).widen(),
        i256([u64::MAX, 0xaaaaaaaaaaaaaaaa, u64::MAX, u64::MAX])
    );
    assert_eq!((-1i128).zero_widen().unsigned(), (u128::MAX).widen());
}

const WORD_LO_MASK: u64 = 0x00000000ffffffff;
const WORD_HI_MASK: u64 = 0xffffffff00000000;
const WORD_FULL_MASK: u64 = 0xffffffffffffffff;
macro_rules! word {
    (1, $val:expr) => {
        (($val >> (32 * 3)) & u128::from(WORD_LO_MASK)) as u64
    };
    (2, $val:expr) => {
        (($val >> (32 * 2)) & u128::from(WORD_LO_MASK)) as u64
    };
    (3, $val:expr) => {
        (($val >> (32 * 1)) & u128::from(WORD_LO_MASK)) as u64
    };
    (4, $val:expr) => {
        (($val >> (32 * 0)) & u128::from(WORD_LO_MASK)) as u64
    };
}

#[test]
fn widen_mul_u128() {
    let a = u128::MAX;
    let b = 2_u128;
    let res = a.zero_widen_mul(b);

    assert_eq!(res, u256([u64::MAX - 1, u64::MAX, 1, 0]));
}
