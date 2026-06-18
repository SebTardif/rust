//@ run-pass
//@ compile-flags: -Zmir-opt-level=2

// Positive test: GVN should still optimize `0 / x` to `0` when `x` is a
// known non-zero constant. This verifies the fix does not regress the
// valid optimization.

fn main() {
    // 0 divided by a known non-zero constant: GVN may fold to 0.
    assert_eq!(0u64 / 1, 0);
    assert_eq!(0u64 / 42, 0);
    assert_eq!(0u64 / u64::MAX, 0);
    assert_eq!(0i32 / 1, 0);
    assert_eq!(0i32 / -1, 0);
    assert_eq!(0i32 / i32::MAX, 0);
    assert_eq!(0i32 / i32::MIN, 0);

    // 0 remainder by a known non-zero constant: GVN may fold to 0.
    assert_eq!(0u64 % 1, 0);
    assert_eq!(0u64 % 42, 0);
    assert_eq!(0i32 % 1, 0);
    assert_eq!(0i32 % -1, 0);

    // Non-zero absorbing elements still work (unrelated to Div/Rem).
    assert_eq!(0u64 * 42, 0);
    assert_eq!(0u64 & 42, 0);
    assert_eq!(0u64 << 5, 0);
    assert_eq!(0u64 >> 5, 0);
}
