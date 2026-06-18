//@ run-pass

// Positive test: NonZero::midpoint() works correctly for both unsigned
// and signed types when the result is non-zero.

use std::num::NonZero;

fn main() {
    // Unsigned: midpoint of two positive NonZero values is always >= 1.
    let u1 = NonZero::<u32>::new(1).unwrap();
    let u4 = NonZero::<u32>::new(4).unwrap();
    assert_eq!(u1.midpoint(u4), NonZero::new(2).unwrap());
    assert_eq!(u4.midpoint(u1), NonZero::new(2).unwrap());

    // Unsigned edge cases.
    let u_max = NonZero::<u32>::new(u32::MAX).unwrap();
    assert_eq!(u1.midpoint(u_max), NonZero::new(u32::MAX / 2 + 1).unwrap());
    assert_eq!(u1.midpoint(u1), u1); // midpoint(1, 1) = 1

    // Signed: same-sign values always produce non-zero midpoint.
    let s3 = NonZero::<i32>::new(3).unwrap();
    let s7 = NonZero::<i32>::new(7).unwrap();
    assert_eq!(s3.midpoint(s7), NonZero::new(5).unwrap());

    // Signed: negative same-sign values.
    let sn3 = NonZero::<i32>::new(-3).unwrap();
    let sn7 = NonZero::<i32>::new(-7).unwrap();
    assert_eq!(sn3.midpoint(sn7), NonZero::new(-5).unwrap());

    // Signed: opposite signs where result is non-zero.
    let s1 = NonZero::<i32>::new(1).unwrap();
    let sn3_b = NonZero::<i32>::new(-3).unwrap();
    assert_eq!(s1.midpoint(sn3_b), NonZero::new(-1).unwrap());

    // i8 edge cases.
    let i8_max = NonZero::<i8>::new(i8::MAX).unwrap();
    let i8_min_plus1 = NonZero::<i8>::new(i8::MIN + 1).unwrap();
    // midpoint(127, -127) = 0, this is the panic case (tested separately)
    // midpoint(127, -126) = 0 (rounds toward negative infinity), also panic case
    // midpoint(127, -125) = 1
    let i8_minus125 = NonZero::<i8>::new(-125).unwrap();
    assert_eq!(i8_max.midpoint(i8_minus125), NonZero::new(1i8).unwrap());
}
