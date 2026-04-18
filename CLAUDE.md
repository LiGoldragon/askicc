# askicc — The Synth Compiler

askicc reads `.synth` dialect files and produces rkyv dialect
data that askic executes as a state machine.

## v0.18 — Four Surfaces

askicc reads from `source/<surface>/*.synth` and produces one
rkyv per surface:

```
source/
  core/    →  generated/dialects.core.rkyv
  aski/    →  generated/dialects.aski.rkyv
  synth/   →  generated/dialects.synth.rkyv
  exec/    →  generated/dialects.exec.rkyv
```

Each subdirectory is a surface. askicc enumerates subdirectories,
resolves each name to a `SurfaceKind` variant, and builds a
`DialectTree { surface, dialects }` for each.

## Synth v0.18 Syntax

**Labels and tags:**
- `@Label` — reads a source token AND identifies output variant
- `:Label` — references an existing name in scope
- `#Tag#` — names output variant WITHOUT reading source token

**Dialect references:**
- `<Name>` — same-surface dialect reference
- `<:surface:Name>` — cross-surface dialect reference

**Literal escapes:** `_@_`, `_$_`, `_+_`, `_*_`, `_?_`, `_:@_`, `_~@_`

**Cardinality:** `*` (zero-or-more), `+` (one-or-more), `?` (optional)

**Delimiters:** `() [] {} (||) [||] {||}`

**Space rules:**
- Space between non-delimiter items = adjacency-optional in source
- No space between non-delimiter items = source must be adjacent
- Space after opening / before closing delimiters = no-op (readability)

**Ordered choice:** Lines prefixed with `//` form an ordered choice.

## Files

- `source/<surface>/*.synth` — grammar per surface
- `src/synth_lex.rs` — tokenizes `.synth` files
- `src/synth_parse.rs` — parses to synth-core types
- `src/main.rs` — enumerates surfaces, emits rkyv per surface

## Rust Style

**No free functions — methods on types always.** `main` is the exception.

## VCS

jj mandatory (git is storage backend only).
