//@ run-fail
//@ error-pattern:midpoint of two NonZero values was zero
//@ needs-subprocess

// Additional regression test: verify the fix works for i8 as well as i32.
// midpoint(1i8, -1i8) = 0, must panic.

use std::num::NonZero;

fn main() {
    let a = NonZero::<i8>::new(1).unwrap();
    let b = NonZero::<i8>::new(-1).unwrap();
    let _ = a.midpoint(b);
}
