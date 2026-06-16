use build_helper::metrics::JsonInvocationSystemStats;
use sysinfo::System;

use super::build_system_stats;

/// Simulates the scenario where `sysinfo` cannot detect any CPUs, which
/// happens in Docker containers with restrictive cgroup configurations,
/// minimal VMs, or CI runners with unusual setups.
///
/// A `System` created without CPU refresh (`System::new()`) reports an
/// empty CPU list, the same as what `sysinfo` returns when the OS does
/// not expose CPU information. Before the fix, `persist()` would panic
/// with an index-out-of-bounds error on `system.cpus()[0]`.
#[test]
fn build_system_stats_no_cpus() {
    // System::new() does not refresh CPU info, so cpus() returns an empty slice.
    // This is the same state sysinfo enters when running in environments
    // where /proc/cpuinfo (Linux) or the equivalent is unavailable.
    let system = System::new();
    assert!(system.cpus().is_empty(), "precondition: no CPUs detected without refresh");

    let stats: JsonInvocationSystemStats = build_system_stats(&system);

    assert_eq!(stats.cpu_threads_count, 0);
    assert_eq!(stats.cpu_model, "unknown");
}
