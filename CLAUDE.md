# askicc — Bootstrap Compiler (crate)

askicc is a Rust crate built into askic (the aski frontend).
It does the heavy lifting at build time so askic is elegant
at runtime.

Sema is the thing. Aski is one text notation. askic is the
frontend that produces .sema. askicc is the layer inside
askic that prepares its type system and grammar.

## What askicc Does (at build time)

1. Reads .synth dialect files → structured grammar data
2. Reads askic's .aski source → scoped Rust types

Both outputs are compiled into the askic binary. At runtime
askic has no files to read except user programs.

## Enum-as-Index Architecture

Generated Rust types mirror aski structure exactly:
- Enum → Rust enum (lookup: which variant?)
- Struct → Rust struct (composite: all fields)
- Module → Rust struct (has enums AND structs AND traits)

Enums are static indexes. O(1) pattern matching. Exhaustive.
Zero strings. The enum IS the hashmap.

## What askicc Contains

- `source/` — .synth dialect files (v0.17: Enum.synth,
  Type.synth, TypeApplication.synth, GenericParam.synth, etc.)
- `src/lexer.rs` — Logos tokenizer (v0.17 tokens)
- `src/lexer_tests.rs` — tests for v0.17 syntax
- `v016_attempt/` — discarded old code (see TERMINOLOGY.md)

## The Layers

```
cc      (aski-core crate)  — .aski → Rust types
askicc  (this crate)       — uses cc + .synth → scoped types + dialects
askic   (askic crate)      — uses askicc → parser, data-tree, .sema
```

askic depends on askicc depends on cc. One binary.

## VCS

Jujutsu (`jj`) mandatory. Git is storage backend only.
Tests in separate files. Domain = any data def (enum + struct + newtype).
