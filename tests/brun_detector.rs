//! Integration-tests voor BRUN-detector.

mod common;

use common::*;
use qbedec::brun::{detect_brun, Confidence};

#[test]
fn detects_brun_with_full_marker_high_confidence() {
    let v = detect_brun(&mini_brun_exe());
    assert!(v.is_brun);
    assert_eq!(v.confidence, Confidence::High);
    assert!(v.evidence.iter().any(|e| e.contains("BRUN45.EXE")));
}

#[test]
fn no_brun_in_plain_mz() {
    let v = detect_brun(&mini_mz());
    assert!(!v.is_brun);
    assert_eq!(v.confidence, Confidence::None);
}

#[test]
fn standalone_large_file_no_brun() {
    let v = detect_brun(&mini_standalone_exe());
    assert!(!v.is_brun);
    assert_eq!(v.confidence, Confidence::None);
}

#[test]
fn brun_marker_only_partial_in_large_file_low_confidence() {
    // Large file met alleen "BRUN45" (zonder ".EXE") — Low confidence (marker maar te groot).
    let mut v = mini_standalone_exe();
    v.extend_from_slice(b"BRUN45");
    let verdict = detect_brun(&v);
    assert_eq!(verdict.confidence, Confidence::Low);
    assert!(!verdict.is_brun); // High+Medium = is_brun true; Low+None = false
}
