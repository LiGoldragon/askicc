# askicc — Bootstrap Compiler

The bootstrap aski compiler. Reads .synth grammar files + askic's
own .aski source (the language anatomy) to build a data-tree with
derived enums. Built once — its output is the fixed foundation
that askic uses to parse any .aski program.

## What askicc Contains

1. The 28 .synth dialect files in `source/` (the grammar)
2. The core .aski definitions — askic's anatomy (domains, structs,
   traits that describe what aski code looks like)
3. The .synth loader (hardcoded parser for .synth files)
4. The declaration-level aski parser ("core aski") — reads modules,
   domains, structs, traits, signatures, imports/exports
5. The lexer and token reader (shared with askic)

askic depends on askicc as a crate and reuses the core parser,
adding body-level parsing (expressions, statements, match arms)
on top.

## askicc vs askic

- **askicc** = bootstrap compiler, written in Rust, reads .aski definitions
- **askic** = the full compiler, uses askicc's data-tree to parse programs
- When askic becomes self-hosted, askicc is the part that gets replaced

## Directory Structure

```
source/           28 .synth dialect files (the grammar data)
examples/hello/   example .aski files demonstrating v0.16 syntax
v016_attempt/     previous attempt with hand-written enums (WRONG approach)
v015_reference/   old kernel.aski + generated Rust (reference only)
```

## VCS

Jujutsu (`jj`) mandatory. Git is storage backend only.
