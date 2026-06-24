// Regression test for #472: struct_lockstep_tails_raw missing ty::Pat arm.
// Before the fix, the function would not recurse through Pat types when
// walking struct tails, potentially producing wrong coercion results.

//@ check-pass

#![feature(pattern_types, pattern_type_macro)]
#![allow(incomplete_features)]

use std::pat::pattern_type;

type NonZeroU32 = pattern_type!(u32 is 1..);

fn accept_nonzero(_: NonZeroU32) {}

fn main() {
    let x: NonZeroU32 = unsafe { std::mem::transmute(1u32) };
    accept_nonzero(x);
}
