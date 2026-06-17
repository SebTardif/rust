#[allow(dead_code)]
pub fn page_size() -> usize {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    assert!(page_size > 0, "sysconf(_SC_PAGESIZE) failed or returned an invalid value");
    page_size as usize
}
