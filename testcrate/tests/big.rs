use compiler_builtins::int::{i256, u256, HInt, MinInt};

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

#[test]
fn widen_mul_u128() {
    let tests = [
        (u128::MAX / 2, 2_u128, u256([u64::MAX, u64::MAX, 0, 0])),
        (u128::MAX, 2_u128, u256([u64::MAX - 1, u64::MAX, 1, 0])),
        (u128::MAX, u128::MAX, u256([1, 0, u64::MAX - 1, u64::MAX])),
        (u128::MIN, u128::MIN, u256::ZERO),
        (1234, 0, u256::ZERO),
        (0, 1234, u256::ZERO),
    ];

    let mut errors = Vec::new();
    for (a, b, exp) in tests {
        let res = a.zero_widen_mul(b);
        if res != exp {
            errors.push((a, b, exp, res));
        }
    }

    for (a, b, exp, res) in errors {
        eprintln!("FAILURE: {a:#036x} * {b:#036x} = {exp:#036x} got {res:#036x}");
    }
    assert!(errors.is_empty());
}

#[test]
fn widen_mul_i128() {
    let tests = [
        (
            i128::MAX / 2,
            2_i128,
            i256([u64::MAX, u64::MAX, u64::MAX, u64::MAX]),
        ),
        (i128::MAX, 2_i128, i256([u64::MAX - 1, u64::MAX, 1, 0])),
        (i128::MIN, 2_i128, i256([u64::MAX - 1, u64::MAX, 1, 0])),
        (i128::MAX, i128::MAX, i256([1, 0, u64::MAX - 1, u64::MAX])),
        (i128::MAX, i128::MAX, i256([1, 0, u64::MAX - 1, u64::MAX])),
        (i128::MIN, i128::MIN, i256::ZERO),
        (1234, 0, i256::ZERO),
        (0, 1234, i256::ZERO),
        (-1234, 0, i256::ZERO),
        (0, -1234, i256::ZERO),
    ];

    let mut errors = Vec::new();
    for (a, b, exp) in tests {
        let res = a.zero_widen_mul(b);
        if res != exp {
            errors.push((a, b, exp, res));
        }
    }

    for (a, b, exp, res) in errors {
        eprintln!("FAILURE: {a:#036x} * {b:#036x} = {exp:#036x} got {res:#036x}");
    }
    assert!(errors.is_empty());
}
