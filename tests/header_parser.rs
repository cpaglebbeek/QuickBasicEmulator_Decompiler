//! Integration-tests voor PE/MZ header parser.

mod common;

use common::*;
use qbedec::pe::{classify, Mz, PeKind};

#[test]
fn parses_mini_mz() {
    let bytes = mini_mz();
    let mz = Mz::parse(&bytes).expect("should parse");
    assert_eq!(mz.e_cp, 0);
    assert_eq!(mz.header_size, 0);
}

#[test]
fn classifies_mini_mz_as_dosmz() {
    assert_eq!(classify(&mini_mz()), PeKind::DosMz);
}

#[test]
fn classifies_mini_pe_as_windows_pe() {
    assert_eq!(classify(&mini_pe_stub()), PeKind::WindowsPe);
}

#[test]
fn classifies_brun_fixture_as_dosmz() {
    assert_eq!(classify(&mini_brun_exe()), PeKind::DosMz);
}

#[test]
fn rejects_truncated_as_invalid() {
    let result = classify(&mini_truncated());
    match result {
        PeKind::Invalid(_) => (),
        other => panic!("expected Invalid, got {:?}", other),
    }
}

#[test]
fn rejects_non_mz_as_invalid() {
    let result = classify(&mini_not_mz());
    match result {
        PeKind::Invalid(msg) => assert!(msg.contains("Not an MZ")),
        other => panic!("expected Invalid, got {:?}", other),
    }
}
