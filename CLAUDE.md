# askicc — The Synth Compiler

askicc reads `.synth` files and produces a single rkyv `dsls.rkyv`
containing every dialect from every DSL — the state-machine data
that askic executes at runtime.

## v0.18 — Four DSLs, one rkyv

askicc reads from `source/<surface>/*.synth` and emits a single
combined `generated/dsls.rkyv` where each `Dialect` carries its
`SurfaceKind`:

```
source/
  core/    \
  aski/     \
  synth/    /  →  generated/dsls.rkyv  (surface-tagged Dialects)
  exec/    /
```

A **DSL** is one of the four surfaces (core, aski, synth, exec).
A **dialect** is one `.synth` file within a DSL (Body, Statement,
Expr, …). askic dispatches by (`SurfaceKind`, `DialectKind`) in one
flat map loaded from `dsls.rkyv`.

## Synth v0.18 Syntax

**Labels and tags (orthogonal):**
- `@Label` — Declare: reads a source token, binds it to a field role
- `:Label` — Reference: reads a source token naming something in scope
- `'Place` — Origin: names a place for lifetime tracking
- `#Tag#` — identifies the output node type; no source read

`@`/`:`/`'` resolve to `LabelKind` + `Binding` (Declare/Reference/Origin).
`#Tag#` resolves to `TagKind`. Separate enums, no overlap.

**Dialect references:**
- `<Name>` — same-surface dialect reference
- `<:surface:Name>` — cross-surface dialect reference

**Literal escapes** (each is one atomic token in source):
`_@_` `_~_` `_$_` `_*_` `_+_` `_?_` `_&_` `_:_` `_<_` `_>_` `_#_` `_//_` `_'_`

Canonical compound forms compose escapes: `_:__@_` (borrow-at),
`_~__@_` (mut-at). No fused `BorrowAt`/`MutAt` tokens — synth is
maximally specific.

**Cardinality:** `*` (zero-or-more), `+` (one-or-more), `?` (optional).

**Delimiters:** `()` `[]` `{}` `(||)` `[||]` `{||}`.

**Space rules:**
- Space between non-delimiter items = adjacency-optional in source
- No space between non-delimiter items = source must be adjacent
- Space after opening or before closing delimiter = ignored

**Ordered choice:** lines prefixed with `//`.

## Files

- `source/<surface>/*.synth` — grammar per DSL
- `src/synth_lex.rs` — tokenizes `.synth` files
- `src/synth_parse.rs` — parses to synth-core types
- `src/main.rs` — enumerates surfaces, emits `dsls.rkyv`

## Rust Style

**No free functions — methods on types always.** `main` is the exception.

## VCS

jj mandatory (git is storage backend only).
