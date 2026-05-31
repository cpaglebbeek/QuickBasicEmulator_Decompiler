# Test Fixtures

Synthetic binary-fixtures voor unit-tests. **Niets is een echte Microsoft binary** — alle bytes zijn handmatig gecraft binnen tests om de PE-parser en BRUN-detector te valideren zonder MS-IP aan te raken (P-QBE-04).

## Hoe het werkt

Tests in `tests/header_parser.rs` en `tests/brun_detector.rs` construeren fixtures **in-memory** via Rust-byte-arrays. We committen geen `.bin`-bestanden in deze repo — alles wordt at-runtime gegenereerd.

## Fixture-types

| Naam | Type | Doel |
|---|---|---|
| `mini_mz()` | 28-byte minimaal-valide DOS-MZ header | Test dat MZ-parser correct werkt |
| `mini_pe_stub()` | DOS-MZ stub + e_lfanew + "PE\0\0" magic | Test dat PE-detector "Windows-PE" rapporteert |
| `mini_brun_exe()` | DOS-MZ stub + literal "BRUN45.EXE" string | Test dat BRUN-detector High-confidence levert |
| `mini_standalone_exe()` | DOS-MZ stub zonder BRUN-string + grootte > 30KB | Test dat BRUN-detector Confidence::None levert |
| `mini_truncated()` | 10 bytes (te klein) | Test error-handling |
| `mini_not_mz()` | "ELF\0..." | Test rejectie van non-MZ binaries |

## Waarom geen committed binaries

- **P-QBE-04**: geen MS-binaries in repo
- **Reproduceerbaarheid**: byte-array-fixtures zijn read-source-of-truth in tests
- **Geen valse positieven door git-LFS-issues**
