use compiler_builtins::int::{i256, u256, HInt};

#[test]
fn widen_u128() {
    assert_eq!(u128::MAX.widen(), u256([u64::MAX, u64::MAX, 0, 0]));
    assert_eq!(
        0xaaaaaaaaaaaaaaaaffffffffffffffff_u128.widen(),
        u256([u64::MAX, 0xaaaaaaaaaaaaaaaa, 0, 0])
    );
}

#[test]
fn widen_i128() {
    assert_eq!(
        (-1i128).widen(),
        i256([u64::MAX, u64::MAX, u64::MAX, u64::MAX])
    );
    assert_eq!(
        (0xaaaaaaaaaaaaaaaaffffffffffffffff_u128 as i128).widen(),
        i256([u64::MAX, 0xaaaaaaaaaaaaaaaa, u64::MAX, u64::MAX])
    );
    assert_eq!((-1i128).zero_widen().unsigned(), (u128::MAX).widen());
}
