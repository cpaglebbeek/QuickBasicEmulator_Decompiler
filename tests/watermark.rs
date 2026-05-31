//! Watermark-tests per P-QBE-05 (verplicht, niet-uitschakelbaar).

use qbedec::watermark_header;

#[test]
fn watermark_contains_required_fields() {
    let h = watermark_header("test.exe", "2026-06-01");
    assert!(h.contains("Reconstructed by QuickBasicEmulator"));
    assert!(h.contains("Source: test.exe"));
    assert!(h.contains("Date: 2026-06-01"));
    assert!(h.contains("WARNING"));
    assert!(h.contains("Verify your legal right"));
}

#[test]
fn watermark_includes_version_and_codename() {
    let h = watermark_header("x", "y");
    assert!(h.contains("Whitten"));
    assert!(h.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn watermark_starts_with_apostrophe_comment_marker() {
    let h = watermark_header("x", "y");
    // Every line of the watermark should start with ' (apostrophe-comment)
    // until the trailing blank line.
    for line in h.lines() {
        if line.trim().is_empty() {
            continue;
        }
        assert!(line.starts_with('\''), "line not a comment: {:?}", line);
    }
}
