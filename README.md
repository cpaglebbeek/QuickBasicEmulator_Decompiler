# QuickBasicEmulator_Decompiler

Decompiler-CLI voor **QuickBASIC 4.5 `.exe`-bestanden**. Twee modes:

| Mode | Sinds | Wat |
|---|---|---|
| **BRUN-mode** | v0.0.4-Whitten (prototype), v0.2.0-Weiland (stabiel) | P-code in BRUN45-linked .exe → `.bas` |
| **Stand-alone** | v0.4.0-Chien | Native x86 met BCOM45-stubs → `.bas` (via signature-DB) |

> ⚠️ **v0.0.1-Gates — Skeleton.** Rust skeleton + clap-CLI. Geen werkende decompilatie nog.

> 💡 **Belangrijk:** QBasic 1.x compileert geen `.exe`. Alleen QuickBASIC 4.5 (+ later QB7/PDS) doet dat. Voor QBasic `.bas`-bestanden: gebruik `_Web` of `_X86`.

## Output-belofte

Decompiler-output is **een leesbare reconstructie**, niet de originele source:

| Wat overleeft | Wat verloren is |
|---|---|
| Control flow (IF/FOR/WHILE/SUB/FUNCTION) | Variabele-namen (worden `V001`, `V002`) |
| Statement-keuzes | Comments (volledig weg) |
| Sub/function-grenzen | Originele indentatie |
| String/numeric literals | Label-namen (worden `L001`) |

Elke output bevat een **watermark-header** (zie [LEGAL.md](https://github.com/cpaglebbeek/Meta_QuickBasicEmulator/blob/main/LEGAL.md)).

## BYO-BRUN45

Voor Stand-alone-mode (v0.4.0+) is een **signature-DB** nodig, opgebouwd uit je eigen QB 4.5 installatie. **Geen MS-binaries in deze repo**. Zie [signatures/README.md](./signatures/README.md).

## Tech

- **Rust** + **serde** + **clap**
- Output-formaten: `.bas` (default) en `.json` (AST)
- Binary-parsing via `object` + `goblin` crates

## Build + run

```bash
cargo build --release
./target/release/qbedec input.exe -o reconstructed.bas
./target/release/qbedec input.exe --format json -o ast.json
```

## Project + ecosystem

- **Meta:** [`cpaglebbeek/Meta_QuickBasicEmulator`](https://github.com/cpaglebbeek/Meta_QuickBasicEmulator)
- **Ecosystem:** Retro_Computing
- **Licentie:** AGPL-3.0
