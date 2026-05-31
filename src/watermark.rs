//! Watermark per P-QBE-05.
//!
//! Elke gegenereerde `.bas` MOET deze header bevatten. **Niet-uitschakelbaar** —
//! er is geen CLI-flag of API om dit te omzeilen. Bewust ontworpen.

use crate::{CODENAME, VERSION};

/// Render the verplichte watermark-header voor decompiler-output.
///
/// `source_filename` = the input .exe naam, `date_iso` = decompilatie-datum in `YYYY-MM-DD`.
pub fn watermark_header(source_filename: &str, date_iso: &str) -> String {
    format!(
        "' Reconstructed by QuickBasicEmulator v{ver}-{cn}\n\
         ' Source: {src}\n\
         ' Date: {dt}\n\
         ' WARNING: Reconstructed source is approximate. Original variable names,\n\
         '          comments and indentation are lost. Verify your legal right to\n\
         '          decompile the input executable. See LEGAL.md.\n\n",
        ver = VERSION,
        cn = CODENAME,
        src = source_filename,
        dt = date_iso,
    )
}
