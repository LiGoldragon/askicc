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

`_` wraps literal aski tokens. Anything between `_..._` is literal —
matched exactly in the aski source. After the closing `_`, synth
notation continues.

| Synth | Literal tokens | Then synth | Aski source |
|-------|---------------|------------|-------------|
| `_@_@name` | `@` | `@name` (declare) | `@myVar` |
| `_:@_Self` | `:@` | `Self` (literal) | `:@Self` |
| `_~@_Self` | `~@` | `Self` (literal) | `~@Self` |
| `_:@_@name` | `:@` | `@name` (declare) | `:@myParam` |
| `_~@_@name` | `~@` | `@name` (declare) | `~@myParam` |
| `_^_<expr>` | `^` | `<expr>` (dialect) | `^expression` |
| `_#_<expr>` | `#` | `<expr>` (dialect) | `#iteration` |

Multi-character literals work naturally:
- `_:@_` = literal `:` then literal `@` (two aski tokens)
- `_@_` = literal `@` (one token)

No `_` needed when there's no collision — bare `@Name` is synth,
bare `Self` is literal.


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

`//` starts each alternative in an ordered choice. First match wins.

Inline `//` for simple either-or:
```
+@export//@Export      ;; either camelCase or PascalCase
```

Line-leading `//` for ordered choice lists:
```
// @_@name :Type /_new (<expr>)
// @_@name /_new (<expr>)
// @_@name :Type
// ~_~@name .set (<expr>)
// ^_<expr>
// <expr>
```

Every alternative starts with `//`. The engine tries them top to
bottom — first match wins. This is the ONLY place where ordered
choice exists in the system.


## Cardinality on Ordered Choice

Each `//` alternative can have its own cardinality prefix, which
controls how many times the engine allows that alternative to match
in a looping context (e.g., root-level file parsing):

```
// *(@Domain/ <domain>)     ;; zero or more domains per file
// *(@trait/ <trait-decl>)   ;; zero or more trait declarations
// ?[|<process>|]            ;; at most one process block per file
```

| Prefix | Meaning |
|--------|---------|
| `*` | Zero or more (can repeat, no limit) |
| `+` | One or more (must appear at least once) |
| `?` | Optional (at most one) |
| (none) | Default: zero or more |

The engine tracks a match count per alternative. When an alternative
reaches its cardinality limit, it is skipped. After the loop ends,
alternatives with `+` cardinality that never matched cause an error.


## Two Parsing Modes

1. **Sequential** — rules without `//`. All apply in order.
   The parser expects each rule to match, one after another.
   ```
   +<param>
   ?:Type
   [<body>]
   ```

2. **Ordered choice** — rules starting with `//`. First match wins.
   The parser tries each alternative top to bottom.
   ```
   // @_@name :Type /_new (<expr>)
   // @_@name /_new (<expr>)
   // <expr>
   ```

Sequential = "expect all of these in order."
Ordered choice = "expect one of these."


## Delimiter Wrapping

The wrapping delimiter in a synth rule IS the token being defined.
A rule wrapped in `()` defines what `()` means. A rule wrapped in
`[]` defines what `[]` means.

```
(@Domain/ <domain>)      ;; ( with Domain key → push domain dialect
[@trait/ <trait-impl>]    ;; [ with trait key → push trait-impl dialect
{@Struct/ <struct>}       ;; { with Struct key → push struct dialect
(|@Ffi/ <ffi>|)          ;; (| with Ffi key → push ffi dialect
[|<process>|]             ;; [| bare → push process dialect
{|@Const/|}              ;; {| with Const key → const (engine-parsed)
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
name. The file contains rules — sequential or ordered choice.

```
;; comments

sequential-rule
sequential-rule

// ordered-choice-alternative
// ordered-choice-alternative
```

No header required. The context is implicit — it's whatever
dialect this file defines.


## Synth Loader

The synth loader is the ONE hardcoded parser in the bootstrap.
It reads `.synth` files and populates AskiWorld's dialect tables.
Everything else is data-driven from the loaded dialects.

Bootstrap sequence:
1. Hardcoded synth loader reads `aski.synth`
2. Loads all referenced inner dialects (`domain.synth`, etc.)
3. AskiWorld now has all dialect tables
4. Parses `.aski` and `.main` files using loaded rules


## AskiWorld Storage

Each loaded dialect becomes entries in AskiWorld:

```
Dialect {
    name: String,
    rules: Vec<Rule>,
}

Rule =
    Sequential { items: Vec<Item> }
    OrderedChoice { alternatives: Vec<ChoiceAlternative> }

ChoiceAlternative = { items: Vec<Item>, cardinality: Card }

Item =
    Placeholder { sigil, casing, name }
    DelimiterRule { delimiter, key, target }
    DialectRef { name }
    Literal { token }
```

The engine walks the rules. Sequential items must all match in order.
Ordered choice tries each alternative top to bottom, first match wins.


## Complete Dialect Set

`aski.synth` — root: module, domains, traits, impls, structs, consts, ffi, process
`module.synth` — exports and imports inside (@Module/ ...)
`domain.synth` — variants inside (@Domain/ ...)
`struct.synth` — fields inside {@Struct/ ...}
`trait-decl.synth` — supertraits and signatures inside (@trait/ ...)
`trait-impl.synth` — type impls inside [@trait/ ...]
`type-impl.synth` — methods inside [@Type/ ...]
`method.synth` — params, return type, body inside (@method/ ...)
`signature.synth` — params and return type inside (@signature/ ...)
`param.synth` — parameter forms (:@Self, ~@Name, @Name :Type, etc.)
`body.synth` — statements inside [ ] or [| |]
`statement.synth` — statement forms (allocation, mutation, return, etc.)
`expr.synth` — expression delimiters (group, inline eval, match, struct construct)
`match.synth` — match target and arms inside (| ... |)
`ffi.synth` — foreign function declarations inside (|@Ffi/ ...|)
`main.synth` — executable files: imports then process
