//! Shared fixtures voor integration-tests.

/// Minimal valid DOS-MZ header (28 bytes, all zeros after the magic).
pub fn mini_mz() -> Vec<u8> {
    let mut v = vec![0u8; 28];
    v[0] = b'M';
    v[1] = b'Z';
    v
}

/// DOS-MZ stub with e_lfanew=0x40 and "PE\0\0" magic at offset 0x40.
pub fn mini_pe_stub() -> Vec<u8> {
    let mut v = vec![0u8; 0x44];
    v[0] = b'M';
    v[1] = b'Z';
    // e_lfanew @ 0x3C = 0x40 (little-endian)
    v[0x3C] = 0x40;
    v[0x3D] = 0x00;
    v[0x3E] = 0x00;
    v[0x3F] = 0x00;
    // PE\0\0 magic at 0x40
    v[0x40] = b'P';
    v[0x41] = b'E';
    v[0x42] = 0;
    v[0x43] = 0;
    v
}

/// DOS-MZ stub with embedded literal "BRUN45.EXE" string.
pub fn mini_brun_exe() -> Vec<u8> {
    let mut v = mini_mz();
    // Pad to 256 bytes then embed marker
    v.extend(vec![0u8; 256 - v.len()]);
    v.extend_from_slice(b"BRUN45.EXE");
    v.extend(vec![0u8; 100]);
    v
}

/// DOS-MZ stub without BRUN-marker, large file size (50KB).
pub fn mini_standalone_exe() -> Vec<u8> {
    let mut v = mini_mz();
    v.extend(vec![0xCCu8; 50 * 1024]);
    v
}

/// Truncated file (less than MZ header size).
pub fn mini_truncated() -> Vec<u8> {
    vec![0u8; 10]
}

/// Non-MZ binary (fake ELF header for sanity).
pub fn mini_not_mz() -> Vec<u8> {
    let mut v = vec![0u8; 32];
    v[0] = 0x7F;
    v[1] = b'E';
    v[2] = b'L';
    v[3] = b'F';
    v
}
