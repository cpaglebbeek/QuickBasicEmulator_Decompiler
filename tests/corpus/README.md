# Test Corpus

Doel: bewijs dat de decompiler werkt op publieke ground-truth.

## Pipeline (vanaf v0.1.x)

```
<test>.bas  →  qb45 /o /x  →  <test>.exe  →  qbedec  →  <test>.reconstructed.bas
                                                       │
                                                       ▼
                                              diff <test>.bas <test>.reconstructed.bas
                                                       │
                                                       ▼
                                              stats.json: % statements matched
```

## Geplande corpus-bestanden (vanaf v0.1.x)

| Bestand | Bron | Status |
|---|---|---|
| `nibbles.bas` | Meegeleverd met QB 1.0/QB 4.5 | TBD: lokaal aanleveren (legal grijs gebied — niet committen) |
| `gorillas.bas` | Meegeleverd met QB 1.0/QB 4.5 | TBD |
| `donkey.bas` | IBM PC ROM BASIC sample | Public — commit OK |
| `hello.bas` | Eigen schrijven | Public — commit OK |
| `loops_demo.bas` | Eigen schrijven | Public — commit OK |
| `subs_demo.bas` | Eigen schrijven | Public — commit OK |

> ⚠️ **Nibbles + Gorillas niet committen** — MS-meegeleverde samples vallen onder MS-copyright. Gebruiker laat zelf staan in `tests/corpus/` lokaal.

## Eigen test-bestanden (commit-veilig)

Komen vanaf v0.0.4-Whitten als onderdeel van BRUN-mode prototype-werk.
