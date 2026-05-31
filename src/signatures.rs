//! Signature-DB schema + load/save.
//!
//! Mapping van byte-patterns naar QB-runtime-functies en hun QB-statement-equivalent.
//! De DB wordt **lokaal door de gebruiker** opgebouwd uit hun eigen QB 4.5 installatie
//! (BYO-BRUN45). Wij distribueren geen MS-binaries (P-QBE-04).
//!
//! Schema is JSON, gevalideerd via serde. Versionering via `schema_version` veld.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureDb {
    pub schema_version: String,
    /// QB 4.5 build-hash that this DB was constructed from (BYO-BRUN45 source).
    /// Empty when not yet built.
    #[serde(default)]
    pub qb45_source_hash: String,
    /// Generation timestamp (RFC3339).
    #[serde(default)]
    pub generated_at: String,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Signature {
    /// Internal runtime-function name (e.g. "B$PRINT", "B$STRA").
    pub function: String,
    /// QB-source-level statement or function this maps to (e.g. "PRINT").
    pub qb_statement: String,
    /// Bytes-pattern in hex (uppercase, no spaces, e.g. "55 8B EC" → "558BEC").
    pub pattern_hex: String,
    /// Optional arg-signature description (e.g. "string" / "number" / "void").
    #[serde(default)]
    pub arg_signature: String,
}

impl Default for SignatureDb {
    fn default() -> Self {
        SignatureDb {
            schema_version: SCHEMA_VERSION.to_string(),
            qb45_source_hash: String::new(),
            generated_at: String::new(),
            signatures: Vec::new(),
        }
    }
}

impl SignatureDb {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let s = fs::read_to_string(path)?;
        let db: SignatureDb = serde_json::from_str(&s)?;
        Ok(db)
    }

    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let s = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, s)?;
        Ok(())
    }

    pub fn add(&mut self, sig: Signature) {
        self.signatures.push(sig);
    }

    pub fn find_by_function(&self, name: &str) -> Option<&Signature> {
        self.signatures.iter().find(|s| s.function == name)
    }
}
