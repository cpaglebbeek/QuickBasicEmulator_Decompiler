//! BRUN-mode detector.
//!
//! QuickBASIC 4.5 BRUN-mode .exe files dynamic-link against `BRUN45.EXE` at runtime.
//! The stub embedded in the .exe typically contains the literal byte string `BRUN45`
//! (and often `BRUN45.EXE`) so the loader can locate the runtime.
//!
//! Heuristic: scan the file for `BRUN45` byte sequence. If present in combination
//! with a small overall file size (typical BRUN-stubs are 1-5KB), classify as BRUN.
//!
//! Stand-alone QB45 .exe files (BCOM45-linked) do NOT reference `BRUN45` literally;
//! they statically embed the runtime stubs (`B$PRINT`, `B$STRA`, etc).
//!
//! Limitations of this prototype (v0.0.4-Whitten):
//!  - No PE-section-aware scanning (we scan whole file as a flat byte array)
//!  - No version-detection (BRUN30/BRUN40 from QB 4.0 share the heuristic)
//!  - False-positive risk: any .exe with "BRUN45" elsewhere (rare in practice)
//!
//! Future (v0.0.5+): combine with relocation-table parsing for higher confidence.

use serde::{Deserialize, Serialize};

const BRUN_MARKER: &[u8] = b"BRUN45";
const BRUN_FULL_MARKER: &[u8] = b"BRUN45.EXE";

/// Threshold (bytes) above which BRUN-mode is unlikely (typical BRUN-stubs are tiny).
const BRUN_SIZE_HEURISTIC: usize = 30 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrunVerdict {
    pub is_brun: bool,
    pub confidence: Confidence,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Confidence {
    High,
    Medium,
    Low,
    None,
}

pub fn detect_brun(bytes: &[u8]) -> BrunVerdict {
    let mut evidence: Vec<String> = Vec::new();

    let has_marker = find_subsequence(bytes, BRUN_MARKER).is_some();
    let has_full_marker = find_subsequence(bytes, BRUN_FULL_MARKER).is_some();
    let size_in_range = bytes.len() <= BRUN_SIZE_HEURISTIC;

    if has_full_marker {
        evidence.push(format!(
            "Embedded literal 'BRUN45.EXE' found at offset {:#x}",
            find_subsequence(bytes, BRUN_FULL_MARKER).unwrap()
        ));
    } else if has_marker {
        evidence.push(format!(
            "Embedded literal 'BRUN45' found at offset {:#x}",
            find_subsequence(bytes, BRUN_MARKER).unwrap()
        ));
    }

    if size_in_range {
        evidence.push(format!(
            "File size {} bytes within BRUN-stub-typical range (<= {} bytes)",
            bytes.len(),
            BRUN_SIZE_HEURISTIC
        ));
    } else {
        evidence.push(format!(
            "File size {} bytes exceeds BRUN-stub-typical range (> {} bytes)",
            bytes.len(),
            BRUN_SIZE_HEURISTIC
        ));
    }

    let confidence = match (has_full_marker, has_marker, size_in_range) {
        (true, _, true) => Confidence::High,
        (true, _, false) => Confidence::Medium,
        (false, true, true) => Confidence::Medium,
        (false, true, false) => Confidence::Low,
        (false, false, _) => Confidence::None,
    };

    let is_brun = matches!(confidence, Confidence::High | Confidence::Medium);

    BrunVerdict {
        is_brun,
        confidence,
        evidence,
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}
