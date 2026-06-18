//@ run-fail
//@ error-pattern:attempt to divide by zero
//@ needs-subprocess
//@ compile-flags: -Zmir-opt-level=2

// Regression test: GVN's absorbing-element optimization incorrectly folded
// `0 / x` to `0` even when `x` might be zero. Division by zero must still
// panic at runtime. This was introduced in rust-lang/rust#125893 (commit
// c3de4b3aad6a) which grouped Div/Rem with Mul/BitAnd/Shl/Shr in a single
// absorbing-element match arm, not accounting for the fact that 0 is NOT an
// absorbing element for division when the divisor is zero.

fn zero_div(x: u64) -> u64 {
    0 / x
}

fn main() {
    // The divisor is zero, so this must panic.
    let _ = zero_div(0);
}
