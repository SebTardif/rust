//@ run-pass
//@ only-linux
// Regression test: poll() revents is a bitmask. The kernel may set
// multiple flags (e.g. POLLIN | POLLHUP) when /dev/random becomes
// ready. Checking with assert_eq! fails when extra flags are set.
// The correct check is a bitwise AND.

fn main() {
    // Simulated revents values the kernel might return.
    let pollin: i16 = 0x0001;  // POLLIN
    let pollhup: i16 = 0x0010; // POLLHUP

    // When only POLLIN is set, both checks pass.
    let revents_simple = pollin;
    assert!(revents_simple & pollin != 0, "POLLIN must be detected");
    assert_eq!(revents_simple, pollin);

    // When POLLIN | POLLHUP is set (common kernel behavior),
    // assert_eq! would fail but bitwise AND still works.
    let revents_combined = pollin | pollhup;
    assert!(revents_combined & pollin != 0, "POLLIN must be detected even with POLLHUP");
    // This would fail: assert_eq!(revents_combined, pollin);
    assert_ne!(revents_combined, pollin, "combined flags differ from POLLIN alone");
}
