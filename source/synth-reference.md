# Synth Specification

Synth is the grammar dialect of aski. Each `.synth` file defines a
dialect — what delimiters mean and what content they expect. The
bootstrap engine reads `.synth` files to build its parsing tables.


## Placeholders

Synth placeholders describe what goes in each position.

| Notation | Meaning |
|----------|---------|
| `@Name` | Declares a new PascalCase name (domain, struct, module, etc.) |
| `@name` | Declares a new camelCase name (trait, method, etc.) |
| `:Name` | References an existing PascalCase name (type, module) |
| `:name` | References an existing camelCase name (import, trait ref) |
| `@value` | A literal value (integer, float, string) |

`@` = declaring. `:` = referencing. Casing mirrors the actual name casing.


## Literal Escape

When aski source has a sigil (`@` `:` `~` `^` `#` `!`) that would
collide with a synth placeholder, use `_` to separate literal from
synth:

| Synth | Means in aski source |
|-------|---------------------|
| `@Name` | Just a name (no literal sigil) |
| `@_@Name` | Literal `@` then a name — instance allocation |
| `:_:Name` | Literal `:` then a name — immutable borrow |
| `~_~Name` | Literal `~` then a name — mutable borrow |
| `^_@expr` | Literal `^` then an expression — return |

Left of `_` = literal aski token. Right of `_` = synth placeholder.
No `_` needed when there's no collision.


## Cardinality

Prefixed on rules or placeholders.

| Prefix | Meaning |
|--------|---------|
| `+` | One or more |
| `*` | Zero or more |
| `?` | Optional (zero or one) |
| (none) | Required (exactly one) |


## Dialect References

| Notation | Meaning |
|----------|---------|
| `<name>` | Push inner dialect. Maps to `name.synth` file. |

Lowercase. When the engine encounters a `<dialect>` reference,
it pushes that dialect onto the dialect stack and parses content
according to that dialect's rules.


## Or

`||` separates alternatives:

```
@export||@Export      ;; either camelCase or PascalCase export
[<body>] || (|<match>|) || [|<body>|] || ___
```


## Delimiter Wrapping

The wrapping delimiter in a synth rule IS the token being defined.
A rule wrapped in `()` defines what `()` means. A rule wrapped in
`[]` defines what `[]` means.

```
(@Domain/ <domain>)     ;; ( with Domain key → push domain dialect
[@trait/ <trait-impl>]   ;; [ with trait key → push trait-impl dialect
{@Struct/ <struct>}      ;; { with Struct key → push struct dialect
(|@Ffi/ <ffi>|)         ;; (| with Ffi key → push ffi dialect
[|<process>|]            ;; [| bare → push process dialect
{|@Const/ :Type @value|} ;; {| with Const key → const (terminal)
```


## Key / Value

`/` separates the key from the value inside a delimited expression.
The key identifies WHAT this expression is. The value is the content.

```
(Element/ Fire Earth Air Water)
 ^^^^^^^ ^^^^^^^^^^^^^^^^^^^^
 key     value
```

In synth: `(@Domain/ <domain>)` — `@Domain` is the key kind,
`<domain>` describes the value.


## File Structure

Each `.synth` file defines one dialect. The filename is the dialect
name. The file contains rules in sequential order — this IS the
parse order.

```
;; comments

rule
rule
rule
```

No header required. No separators between rules. The context is
implicit — it's whatever dialect this file defines.


## Synth Loader

The synth loader is the ONE hardcoded parser in the bootstrap.
It reads `.synth` files and populates AskiWorld's dialect tables.
Everything else is data-driven from the loaded dialects.

Bootstrap sequence:
1. Hardcoded synth loader reads `aski.synth`
2. Loads all referenced inner dialects (`domain.synth`, etc.)
3. AskiWorld now has all transition tables
4. Parses `.aski` and `.main` files using loaded rules


## AskiWorld Storage

Each loaded dialect becomes entries in AskiWorld:

```
Dialect {
    name: String,
    rules: Vec<Rule>,
}

Rule {
    delimiter: TokenKind,     ;; which delimiter this rule is about
    key_kind: KeyKind,        ;; what the key looks like
    cardinality: Cardinality, ;; + * ? or required
    target: Target,           ;; dialect ref or terminal
}
```

The engine sees a delimiter, reads the key, looks up the matching
rule, and either pushes a dialect or parses terminal content.


## Complete Example

`aski.synth`:
```
(@Module/ <module>)

*(@Domain/ <domain>)
*(@trait/ <trait-decl>)

*[@trait/ <trait-impl>]

*{@Struct/ <struct>}

*{|@Const/ :Type @value|}

*(|@Ffi/ <ffi>|)

?[|<process>|]
```

This defines: an aski file starts with a module declaration,
then any number of domains, traits, impls, structs, consts,
FFI blocks, and optionally an inline process.

`domain.synth`:
```
+@Variant
*(@Variant/ :Type)
*{@Variant/ <struct>}
```

This defines: inside a domain, one or more variant names,
with optional data-carrying `()` or struct `{}` variants.

`module.synth`:
```
+@export||@Export
*[:Module/ +:import||:Import]
```

This defines: inside a module header, one or more exports
(either casing), then zero or more import blocks.
