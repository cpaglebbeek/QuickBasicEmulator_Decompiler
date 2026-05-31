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

## Schema (preview voor v0.4.0)

```json
{
  "version": "1.0",
  "qb45_revision": "4.50 (build hash)",
  "signatures": [
    {
      "function": "B$PRINT",
      "qb_statement": "PRINT",
      "pattern_hex": "...",
      "arg_signature": "string|number"
    }
  ]
}
```

## Notities

- Signatures zijn machine-specifiek (compiler-build kan verschillen). Daarom lokaal opbouwen.
- Decompiler-CLI faalt gracefully als DB niet aanwezig: foutmelding wijst naar deze README.
