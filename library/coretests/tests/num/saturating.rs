use core::num::Saturating;

/// Regression test: Saturating<iN> % Saturating(-1) must not panic when
/// the dividend is iN::MIN. Before the fix, Saturating used `self.0.rem(other.0)`,
/// which panics on `iN::MIN % -1` because the mathematical result (0) is fine
/// but the primitive `%` goes through the division instruction which traps on
/// overflow in debug mode. The fix uses `wrapping_rem` instead.
/// See https://github.com/SebTardif/rust/issues/423
macro_rules! saturating_rem_min_test {
    ($fn_name:ident, $t:ty) => {
        #[test]
        fn $fn_name() {
            let min = Saturating(<$t>::MIN);
            let neg_one = Saturating(-1 as $t);
            let zero = Saturating(0 as $t);

            // MIN % -1 must equal 0, not panic
            assert_eq!(min % neg_one, zero);

            // Also verify normal cases still work
            let five = Saturating(5 as $t);
            let three = Saturating(3 as $t);
            let two = Saturating(2 as $t);
            assert_eq!(five % three, two);
        }
    };
}

saturating_rem_min_test!(test_saturating_rem_i8, i8);
saturating_rem_min_test!(test_saturating_rem_i16, i16);
saturating_rem_min_test!(test_saturating_rem_i32, i32);
saturating_rem_min_test!(test_saturating_rem_i64, i64);
saturating_rem_min_test!(test_saturating_rem_i128, i128);
saturating_rem_min_test!(test_saturating_rem_isize, isize);
