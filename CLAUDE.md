# CLAUDE.md — QuickBasicEmulator_Decompiler

## Rol

Decompiler voor QB 4.5 .exe → .bas. Greenfield-werk (geen herbruikbare open-source decompiler bestaat).

## Sessie-startprotocol

1. Pull deze repo + Meta_QuickBasicEmulator
2. Lees ROADMAP
3. `cargo build` voor toolchain-check

## Belangrijke regels

### BYO-BRUN45 (legal)

- NOOIT MS-binaries (BRUN45.EXE, BCOM45.LIB) in deze repo committen
- Signature-DB wordt **lokaal** opgebouwd op gebruikers-machine uit hun eigen QB-installatie
- Zie `signatures/README.md` voor de bouw-instructies (te schrijven in v0.0.4-Whitten)

### Watermark verplicht

- Elke gegenereerde `.bas` output MOET de watermark-header bevatten zoals gedefinieerd in `Meta_QuickBasicEmulator/LEGAL.md`
- Het watermark is **niet verwijderbaar via een vlag**

### Test-corpus discipline

Conform B6-beslissing: `tests/corpus/` bevat publieke open-source QB-programs als ground-truth. Pipeline:
1. `.bas` → `qb45 /o /x` → `.exe` (BYO-toolchain)
2. `qbedec` → `.bas'` reconstructed
3. Diff-rapport: % statements correct gereconstrueerd

## Code-locaties

| Wat | Waar |
|---|---|
| Entry point | `src/main.rs` |
| BRUN-mode parser | `src/brun/` (v0.0.4+) |
| Stand-alone parser | `src/standalone/` (v0.4.0+) |
| Signature-DB schema | `src/signatures/schema.rs` |
| Test-corpus | `tests/corpus/` |

## CI-status

`.github/_workflows_pending/ci.yml` ligt klaar maar is nog niet actief op GitHub. Reden: bij eerste push had `gh` OAuth-token de `workflow` scope niet. Activeren met:
```bash
gh auth refresh -s workflow
mv .github/_workflows_pending .github/workflows
git add -A && git commit -m "Activate CI workflow" && git push
```
