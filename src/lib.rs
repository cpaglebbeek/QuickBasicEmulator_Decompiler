//! qbedec — QuickBASIC 4.5 decompiler library
//!
//! v0.0.4-Whitten — BRUN-mode prototype + signature-DB schema + watermark.
//!
//! Modules:
//!  - pe          : DOS-MZ + PE header parser (minimal, no external deps)
//!  - brun        : BRUN-mode detector (heuristics on header + embedded strings)
//!  - signatures  : Signature-DB schema + load/save (JSON)
//!  - ast         : Rust mirror of Core AST-types for output
//!  - watermark   : Non-bypassable watermark per P-QBE-05

pub mod pe;
pub mod brun;
pub mod signatures;
pub mod ast;
pub mod watermark;

pub use pe::{Mz, PeKind};
pub use brun::{BrunVerdict, detect_brun};
pub use signatures::SignatureDb;
pub use watermark::watermark_header;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CODENAME: &str = "Whitten";
