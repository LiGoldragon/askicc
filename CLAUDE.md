# askicc — Bootstrap Compiler

askicc is a binary that reads .synth dialect files, populates
a domain-data-tree of aski-core types, and serializes it as
rkyv. askic reads this rkyv data to know how to parse using
a dialect-based state machine.

## What askicc Does

Reads .synth dialect files → populates a domain-data-tree →
serializes as rkyv.

**askicc does NOT generate Rust code.** Only cc and semac
generate Rust. askicc produces rkyv-serialized data that gets
embedded in the askic binary at build time. The domain-data-
tree IS the state machine that drives askic's parser.

## Shared Types from aski-core

askicc depends on cc's generated Rust types (from aski-core).
These types have rkyv derives so askicc can serialize them.
askic depends on the same types to deserialize.

aski-core is the rkyv contract — it defines every type that
appears in the message between askicc and askic.

askicc populates instances of these types by reading .synth
files. The populated tree captures all grammar knowledge that
askic needs: what tokens to match, in what order, with what
adjacency, using what delimiters, with what cardinality.

## The Pipeline

```
cc       — .aski → Rust types (bootstrap seed)
askicc   — .synth → rkyv domain-data-tree (this binary)
askic    — reads rkyv data-tree → dialect state machine → rkyv parse tree
semac    — reads rkyv → produces sema + Rust
```

Four separate binaries. They communicate through files.

## What askicc Contains

- `source/` — .synth dialect files (31 files, v0.17)
- `aski/` — .aski domain definition files (8 files)
- `src/synth_lex.rs` — synth tokenizer
- `src/synth_parse.rs` — .synth → Dialect domain instances
- `src/aski_parse.rs` — .aski → domain definitions

## Rust Style

**No free functions — methods on types always.** All Rust
will eventually be rewritten in aski, which uses methods
(traits + impls). `main` is the only exception.

## VCS

Jujutsu (`jj`) mandatory. Git is storage backend only.
Tests in separate files. Domain = any data def (enum + struct + newtype).
