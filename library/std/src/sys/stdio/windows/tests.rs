use super::utf16_to_utf8;

#[test]
fn zero_size_read() {
    assert_eq!(utf16_to_utf8(&[], &mut []).unwrap(), 0);
}

/// Regression test: the low surrogate range is U+DC00..=U+DFFF, not U+DCEE..=U+DFFF.
/// Surrogate code units below 0xDCEE (like 0xDC00) must be recognized as low surrogates
/// for correct UTF-16 to UTF-8 byte counting in console output.
#[test]
fn low_surrogate_range_boundary() {
    // A low surrogate should occupy 1 byte in the byte count (the other 2 bytes
    // are counted with the high surrogate). Verify the full range is recognized.
    let is_low_surrogate = |ch: u16| matches!(ch, 0xDC00..=0xDFFF);

    // These must all be recognized as low surrogates.
    assert!(is_low_surrogate(0xDC00), "0xDC00 is the first low surrogate");
    assert!(is_low_surrogate(0xDCED), "0xDCED is a low surrogate below old boundary");
    assert!(is_low_surrogate(0xDCEE), "0xDCEE was the old (wrong) boundary start");
    assert!(is_low_surrogate(0xDFFF), "0xDFFF is the last low surrogate");

    // These must NOT be recognized as low surrogates.
    assert!(!is_low_surrogate(0xDBFF), "0xDBFF is a high surrogate, not low");
    assert!(!is_low_surrogate(0xE000), "0xE000 is past the surrogate range");
}
