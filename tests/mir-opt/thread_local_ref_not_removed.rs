//@ test-mir-pass: SimplifyLocals-before-const-prop
// Regression test: ThreadLocalRef must not be eliminated by dead store
// optimization. Removing it can cause SIGILL when the thread-local
// initializer is skipped.

#![feature(thread_local)]

#[thread_local]
static mut TLS_VAR: u32 = 0;

// EMIT_MIR thread_local_ref_not_removed.access_tls.SimplifyLocals-before-const-prop.diff
fn access_tls() {
    // This access looks unused, but the ThreadLocalRef rvalue must be
    // preserved because it has a runtime side effect (executing the
    // thread-local initializer).
    unsafe { TLS_VAR };
}

fn main() {
    access_tls();
}
