//@ run-fail
//@ error-pattern:remainder with a divisor of zero
//@ needs-subprocess
//@ compile-flags: -Zmir-opt-level=2

// Regression test: same root cause as gvn-zero-div-by-zero-panics.rs.
// `0 % x` must panic when `x` is zero, but GVN folded it to constant 0.

fn zero_rem(x: u64) -> u64 {
    0 % x
}

fn main() {
    let _ = zero_rem(0);
}
