# askicc — The Synth Compiler

askicc reads `.synth` files and produces a single rkyv `dsls.rkyv`
containing every dialect from every DSL — the state-machine data
that askic executes at runtime.

## v0.20 — Five DSLs, one rkyv

askicc reads from `source/<surface>/*.synth` and emits a single
combined `generated/dsls.rkyv` where each `Dialect` carries its
`SurfaceKind`:

```
source/
  core/    \
  aski/     \
  synth/     \  →  generated/dsls.rkyv  (surface-tagged Dialects)
  exec/     /
  rfi/     /
```

A **DSL** is one of the five surfaces (core, aski, synth, exec, rfi).
A **dialect** is one `.synth` file within a DSL (Body, Statement,
Expr, …). askic dispatches by (`SurfaceKind`, `DialectKind`) in one
flat map loaded from `dsls.rkyv`.

Current state: **44 dialects** across 5 DSLs (Core=3, Aski=32, Synth=6, Exec=2, Rfi=1).
**53 tests pass** — 30 v0.18 regression tests + 19 v0.19-specific tests + 4 v0.20-specific tests
(borrow shapes, mutable borrow, type-app brace, LocalDecl tags, VariantAlt,
path separator, generic slot, exec Program tag, wildcard pattern).

## Synth v0.20 Syntax

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

**Literal escapes (aski-source tokens):**
- `_@_` `_~_` `_$_` `_*_` `_+_` `_?_` `_&_` `_:_` `_<_` `_>_` `_#_` `_//_` `_'_`

Each is one atomic token. Compound forms compose: `_~__&_` is `~` adjacent to `&` (mutable borrow).

## v0.20 Aski Syntax (what the .synth files encode)

- **Visibility:** `@` prefix on declarations and field slots = public; default private (v0.20 new).
- **Trait decl at `[|...|]`** delimiter (v0.20 — was `(...)` before; reclaimed from RFI).
- **RFI:** moved to its own `.rfi` surface (v0.20).
- **Associated types:** `Item` bare in trait decl; `(Item Token)` in impl; `self:Item` path (v0.20).
- **Self in expressions:** `self.field`, `self.method()` work (v0.20 added `#SelfRef#` to ExprAtom).
- **Default trait methods:** Method body is `?<MethodBody>` (v0.19) — missing body = required.
- **Borrow:** `&self` (shared), `~&self` (mutable). Was `:@Self` / `~@Self` in v0.18.
- **Path:** `Type:method(args)`, `Type:Variant`. Was `Type/method`, `Type/Variant`.
- **Type application:** `{Vec Element}`. Was `[Vec Element]`.
- **Or-pattern:** `[Fire Air]`. Was `(Fire | Air)`.
- **Generic slot:** `{$Value}` after definition name; bound set `$Value{Clone Debug}`.
- **Local decl:** `(counter U32:new(0))` — 5 shapes via `()`. Was `@Counter U32/new(0)`.
- **ExprStmt:** `[expr]` for side-effects. Was bare expression.
- **Case rule:** Pascal for compile-time structural (incl. traits); camel for actual instances of a type (incl. locals, methods, self, match-arm bindings). `F64` is the type; `f64` is an instance of it.
- **Retired:** `@` sigil, `&` combinator, `Self` keyword (now `self`).

See `/home/li/git/aski/spec/syntax-v020.aski` for the full language-by-example.
