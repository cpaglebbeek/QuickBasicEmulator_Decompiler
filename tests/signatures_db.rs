//! Tests voor Signature-DB schema + round-trip.

use qbedec::signatures::{Signature, SignatureDb, SCHEMA_VERSION};
use std::fs;

#[test]
fn empty_db_has_correct_schema_version() {
    let db = SignatureDb::empty();
    assert_eq!(db.schema_version, SCHEMA_VERSION);
    assert!(db.signatures.is_empty());
}

#[test]
fn add_and_find_by_function() {
    let mut db = SignatureDb::empty();
    db.add(Signature {
        function: "B$PRINT".to_string(),
        qb_statement: "PRINT".to_string(),
        pattern_hex: "558BEC".to_string(),
        arg_signature: "string".to_string(),
    });
    let found = db.find_by_function("B$PRINT").unwrap();
    assert_eq!(found.qb_statement, "PRINT");
}

#[test]
fn json_roundtrip() {
    let tmp = std::env::temp_dir().join("qbedec_sig_test.json");
    let mut db = SignatureDb::empty();
    db.qb45_source_hash = "abc123".to_string();
    db.add(Signature {
        function: "B$STRA".to_string(),
        qb_statement: "STR$".to_string(),
        pattern_hex: "ABCDEF".to_string(),
        arg_signature: "number".to_string(),
    });
    db.save_to_file(&tmp).unwrap();
    let loaded = SignatureDb::load_from_file(&tmp).unwrap();
    assert_eq!(loaded.qb45_source_hash, "abc123");
    assert_eq!(loaded.signatures.len(), 1);
    assert_eq!(loaded.signatures[0].pattern_hex, "ABCDEF");
    let _ = fs::remove_file(&tmp);
}
