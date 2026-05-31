# Signatures — BYO-BRUN45

Voor de Stand-alone EXE-mode (vanaf v0.4.0-Chien) heeft de decompiler een **signature-DB** nodig: een mapping van byte-patterns naar QB-statements/runtime-functies in `BRUN45.EXE` / `BCOM45.LIB`.

## Wij distribueren GEEN Microsoft-binaries.

Dat is een bewuste beleidskeuze om alle IP-discussie te vermijden. Zie [Meta_QuickBasicEmulator/LEGAL.md](https://github.com/cpaglebbeek/Meta_QuickBasicEmulator/blob/main/LEGAL.md).

## Hoe je je eigen signature-DB bouwt (vanaf v0.4.0)

1. Zorg dat je een **legitieme installatie** hebt van QuickBASIC 4.5 (eigen kopie, archive.org, etc.).
2. Plaats `BRUN45.EXE` en `BCOM45.LIB` in `~/.qbedec/qb45/`.
3. Run:
   ```bash
   qbedec --rebuild-signatures
   ```
4. De DB komt in `~/.qbedec/signatures.json` (lokaal, niet gedeeld).

## Schema (vanaf v0.0.4-Whitten)

Officieel JSON-schema: zie [`schema.json`](./schema.json). Voorbeeld:

```json
{
  "schema_version": "1.0",
  "qb45_source_hash": "<sha256 van BRUN45.EXE>",
  "generated_at": "2026-06-01T12:00:00Z",
  "signatures": [
    {
      "function": "B$PRINT",
      "qb_statement": "PRINT",
      "pattern_hex": "558BEC",
      "arg_signature": "string"
    }
  ]
}
```

### CLI-roundtrip (vanaf v0.0.4)

De Rust-library (`qbedec`) exposes load/save:

```rust
use qbedec::signatures::{SignatureDb, Signature};

let mut db = SignatureDb::empty();
db.add(Signature {
    function: "B$PRINT".into(),
    qb_statement: "PRINT".into(),
    pattern_hex: "558BEC".into(),
    arg_signature: "string".into(),
});
db.save_to_file(std::path::Path::new("~/.qbedec/signatures.json"))?;
```

### CLI `--rebuild-signatures` (gepland v0.4.0-Chien)

```bash
qbedec --rebuild-signatures --brun-source ~/qb45/BRUN45.EXE
```

## Notities

- Signatures zijn machine-specifiek (compiler-build kan verschillen). Daarom lokaal opbouwen.
- Decompiler-CLI faalt gracefully als DB niet aanwezig: foutmelding wijst naar deze README.
- DB bestand komt in `~/.qbedec/signatures.json` (niet in repo, niet gedeeld tussen gebruikers).
