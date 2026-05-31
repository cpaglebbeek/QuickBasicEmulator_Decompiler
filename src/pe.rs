//! DOS-MZ + PE header parser (minimal, vanilla Rust).
//!
//! QuickBASIC 4.5 produces DOS-MZ executables (16-bit real-mode). PE-format is irrelevant
//! for QB45 itself, but we expose it for completeness and to give clear "not-a-QB45-exe"
//! diagnostics when users feed in modern .exes.
//!
//! Spec-referentie: MS-DOS Programmer's Reference + Microsoft PE Format docs.
//! Geen verbatim MS-code-port.

use anyhow::{anyhow, Result};

/// Minimum size of a valid DOS-MZ header (28 bytes).
const MZ_HEADER_SIZE: usize = 28;

/// "MZ" signature at offset 0.
pub const MZ_MAGIC: [u8; 2] = [0x4D, 0x5A];

/// "PE\0\0" signature at offset e_lfanew (only in PE/Windows EXEs, not DOS-MZ).
pub const PE_MAGIC: [u8; 4] = [b'P', b'E', 0, 0];

#[derive(Debug, Clone)]
pub struct Mz {
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    /// Header size in bytes (e_cparhdr * 16).
    pub header_size: usize,
    /// Estimated file size from MZ-header (e_cp * 512 - (512 - e_cblp)).
    pub mz_file_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeKind {
    /// Pure DOS-MZ executable (real-mode 16-bit). What QB45 produces.
    DosMz,
    /// PE/Windows executable with DOS-MZ stub. Not produced by QB45 — modern toolchain output.
    WindowsPe,
    /// MZ header malformed or file truncated.
    Invalid(String),
}

impl Mz {
    /// Parse the MZ header from a byte slice. Returns Err if not an MZ at all.
    pub fn parse(bytes: &[u8]) -> Result<Mz> {
        if bytes.len() < MZ_HEADER_SIZE {
            return Err(anyhow!(
                "File too small to contain MZ header: got {} bytes, need >= {}",
                bytes.len(),
                MZ_HEADER_SIZE
            ));
        }
        if bytes[0..2] != MZ_MAGIC {
            return Err(anyhow!(
                "Not an MZ executable (magic = {:02X} {:02X})",
                bytes[0],
                bytes[1]
            ));
        }
        let u16le = |off: usize| u16::from_le_bytes([bytes[off], bytes[off + 1]]);

        let e_cblp = u16le(2);
        let e_cp = u16le(4);
        let e_cparhdr = u16le(8);
        let header_size = (e_cparhdr as usize) * 16;
        let mz_file_size = if e_cp == 0 {
            0
        } else {
            // (e_cp - 1) * 512 + e_cblp ; if e_cblp == 0, the last page is full (512 bytes).
            let last_page = if e_cblp == 0 { 512 } else { e_cblp as usize };
            ((e_cp as usize) - 1) * 512 + last_page
        };

        Ok(Mz {
            e_cblp,
            e_cp,
            e_crlc: u16le(6),
            e_cparhdr,
            e_minalloc: u16le(10),
            e_maxalloc: u16le(12),
            e_ss: u16le(14),
            e_sp: u16le(16),
            e_csum: u16le(18),
            e_ip: u16le(20),
            e_cs: u16le(22),
            e_lfarlc: u16le(24),
            e_ovno: u16le(26),
            header_size,
            mz_file_size,
        })
    }
}

/// Classify the executable kind by inspecting MZ + PE-stub.
pub fn classify(bytes: &[u8]) -> PeKind {
    let mz = match Mz::parse(bytes) {
        Ok(m) => m,
        Err(e) => return PeKind::Invalid(e.to_string()),
    };

    // If file is long enough to contain e_lfanew (at offset 0x3C) and a PE-magic
    // at that offset, it's a Windows PE wrapped in DOS-MZ stub.
    if bytes.len() >= 0x40 {
        let e_lfanew = u32::from_le_bytes([bytes[0x3C], bytes[0x3D], bytes[0x3E], bytes[0x3F]])
            as usize;
        if e_lfanew + 4 <= bytes.len() {
            if &bytes[e_lfanew..e_lfanew + 4] == PE_MAGIC {
                return PeKind::WindowsPe;
            }
        }
    }

    // Sanity: if MZ-reported size is much larger than the file, that's suspicious.
    let _ = mz; // we keep parsing strict but don't fail on size mismatch here.

    PeKind::DosMz
}
