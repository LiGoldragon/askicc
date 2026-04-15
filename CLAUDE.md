# askicc — Bootstrap Compiler

The bootstrap compiler of the sema engine. Reads .synth grammar files
+ aski-core .aski definitions (the language anatomy) to build a
data-tree with derived enums.

Built **once** — its binary output is what askic uses. Not re-run
per compilation.

## What askicc Does

1. Reads the 28 PascalCase .synth dialect files in `source/`
   (filenames = DialectKind variant names)
2. Reads the aski-core .aski definitions (from the aski-core repo)
3. Produces a data-tree: every construct, name, relationship

## What askicc Contains

- `source/` — 28 PascalCase .synth dialect files (the grammar)
- `src/synth/loader.rs` — hardcoded .synth parser
- `src/synth/types.rs` — Dialect, Rule, Item, Card, Delimiter
- `src/lexer.rs` — Logos tokenizer (42 token variants)
- `src/engine/tokens.rs` — TokenReader
- `v016_attempt/` — previous attempt with hand-written enums (WRONG)
- `v015_reference/` — old kernel.aski + generated Rust

## The Sema Engine

```
aski-core  →  askicc  →  askic  →  semac
(anatomy)    (bootstrap)  (compiler)  (sema gen)
```

Bootstrap = Rust. Self-hosted = all Rust rewritten in aski.

## VCS

Jujutsu (`jj`) mandatory. Git is storage backend only.
