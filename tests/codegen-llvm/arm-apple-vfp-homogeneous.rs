// Regression test for #471: ARM VFP detection used triple suffix "hf" check
// which missed Apple ARM hard-float targets (e.g., armv7-apple-ios).
// The fix uses llvm_floatabi instead.
//
// This test verifies that homogeneous float aggregates are passed in VFP
// registers (s0-s3) on Apple ARM targets, not on the stack.

//@ compile-flags: --target armv7-apple-ios
//@ needs-llvm-components: arm
//@ min-llvm-version: 18

#![feature(no_core, lang_items)]
#![no_core]
#![no_std]

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

impl Copy for f32 {}

#[repr(C)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

// CHECK-LABEL: @pass_float2
// CHECK: float %arg.0, float %arg.1
#[no_mangle]
pub extern "C" fn pass_float2(arg: Float2) -> f32 {
    arg.x
}
