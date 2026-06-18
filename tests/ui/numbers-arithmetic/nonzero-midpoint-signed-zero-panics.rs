//@ run-fail
//@ error-pattern:midpoint of two NonZero values was zero
//@ needs-subprocess

// Regression test: NonZero::midpoint() used `new_unchecked` in a macro
// that applied to both signed and unsigned types. The SAFETY comment
// claimed "impossible because of the unsignedness of this number," but
// the macro also generates signed variants. For signed NonZero types,
// two values with opposite signs can produce a midpoint of exactly zero,
// creating a NonZero with value 0 (undefined behavior).
//
// Introduced in rust-lang/rust#92048 (commit 18bfe5d8a9c) which added
// the midpoint method. The SAFETY comment by David Tolnay (commit
// b21b9cc901c3) only considered unsigned types. Stabilized in Rust 1.85.0
// (unsigned) and 1.87.0 (signed), making the UB reachable from stable
// safe Rust.

use std::num::NonZero;

fn main() {
    let a = NonZero::<i32>::new(1).unwrap();
    let b = NonZero::<i32>::new(-1).unwrap();
    // midpoint(1, -1) = 0, which violates NonZero invariant.
    // After the fix, this must panic instead of creating UB.
    let _ = a.midpoint(b);
}
