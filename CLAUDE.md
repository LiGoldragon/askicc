# synthc — Synth Dialect Definitions

The 28 synth dialect files that define the entire aski v0.16 language.
Synth IS the grammar — no engine special cases.

## Directory Structure

```
source/
  aski.synth          — root dialect (module, domain, trait, struct, const, ffi, process)
  module.synth        — module header (exports, imports)
  domain.synth        — domain variants (nullary, data, struct)
  struct.synth        — struct fields (typed, self-typed)
  trait-decl.synth    — trait declarations with signatures
  trait-impl.synth    — trait implementations per type
  type-impl.synth     — methods inside a type impl
  method.synth        — method params, return type, body
  signature.synth     — trait signature (params + return type)
  param.synth         — parameter forms (borrow, mut, owned, named)
  body.synth          — statements + tail expression
  statement.synth     — early return, loop, iteration, mutation, allocation, expr
  loop.synth          — conditional and infinite loops
  allocation.synth    — typed, initialized, bare allocations
  mutation.synth      — method-call, type, init mutations
  match.synth         — match arms
  pattern.synth       — variant, literal, wildcard, or patterns
  ffi.synth           — FFI declarations
  main.synth          — main entry point
  process.synth       — process declarations
  expr.synth          — expression entry (→ expr-or)
  expr-or.synth       — || precedence
  expr-and.synth      — && precedence
  expr-compare.synth  — == != < > <= >= precedence
  expr-add.synth      — + - precedence
  expr-mul.synth      — * % precedence
  expr-postfix.synth  — .field .method() ? precedence
  expr-atom.synth     — literals, refs, groups, inline eval, struct construct

examples/hello/       — example .aski files using the v0.16 syntax
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
