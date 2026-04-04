# aski-core — Kernel Schema

The contract between aski-rs (Rust backend) and aski-cc (aski compiler).

## Source of Truth

`source/kernel.aski` defines ALL relation types (domains, structs).
build.rs compiles kernel.aski to Rust types via aski-rs.

**Do not hand-write Rust types for relations.** If a new relation is needed:
1. Add it to `source/kernel.aski`
2. Rebuild — build.rs compiles kernel.aski → generated Rust types
3. Use the generated types in the Ascent World

The only hand-written Rust allowed:
- `build.rs` — invokes aski compiler on kernel.aski
- Ascent derivation rules in `src/lib.rs` — temporary, will move to aski
- Query functions in `src/lib.rs` — thin wrappers over Ascent relations

## VCS

Jujutsu (`jj`) mandatory. Git is storage backend only.

## Language Policy

Rust only for application logic. Nix only for builds.

## Directory Structure

- `source/kernel.aski` — **the source of truth** for all relation types
- `src/lib.rs` — Ascent World + derivation rules + queries (uses generated types)
