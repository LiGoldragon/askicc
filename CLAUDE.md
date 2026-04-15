# synthc — Stage 1: Synth Compiler

Reads .synth dialect files + .aski source. Builds a fully-explicit
data-tree. Derives per-kind enums from discovered symbol names.

## What synthc Does

Stage 1 of the pipeline. Reads two inputs:
1. The 28 .synth dialect files in `source/` (the grammar)
2. The .aski source from askic (the code)

From these it generates a data-tree — every construct, every name,
every relationship spelled out. The symbol names become variant names
of derived enums. The enums come FROM the data, not from a hand-written
list. This data-tree is the foundation of askic's parsing state machine.

## Directory Structure

```
source/           28 .synth dialect files (the grammar data)
examples/hello/   example .aski files demonstrating v0.16 syntax
v015_reference/   old kernel.aski + generated Rust (reference only)
```

## Dialect Tree

```
aski (root)
├── module, domain, struct (type declarations)
├── trait-decl → signature → param
├── trait-impl → type-impl → method → param, body, match
├── body → statement → allocation, mutation, loop, match, expr
└── expr → expr-or → expr-and → expr-compare → expr-add
         → expr-mul → expr-postfix → expr-atom
```

## VCS

Jujutsu (`jj`) mandatory. Git is storage backend only.
