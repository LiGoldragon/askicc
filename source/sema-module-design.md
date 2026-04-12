# Sema Module Design

## The Problem

Right now, every change recompiles everything. A single-crate Rust
output means the entire SemaWorld becomes one .rs file. Change one
variant → recompile everything.

## The Goal

Each aski module becomes a separate unit in Sema. When a module
changes, only that module and its dependents recompile. Eventually
each compiled module is content-addressed in criome-store — if the
content hasn't changed, the compiled artifact is reused.

## What a Sema Module Is

A SemaModule is the compiled form of one .aski file. It contains:

- Types (domains, structs) declared in this module
- Traits declared in this module
- Trait implementations defined in this module
- FFI bindings declared in this module
- Constants declared in this module
- Its dependency graph (which other SemaModules it imports)
- Its export list (which names are visible to importers)

A SemaModule does NOT contain:
- Source text (that's AskiWorld's job)
- Parse nodes (that's AskiWorld's job)
- Anything from imported modules (only references by ID)

## How Modules Map to Rust Crates

Each SemaModule → one Rust crate:

```
(Parser/ domainDecl item [Core/ Element Quality])
→ crate parser_gen {
    use core_gen::{Element, Quality};
    ...
}
```

Dependencies between crates mirror the import graph:
```
core.aski        → core_gen (no deps)
parser.aski      → parser_gen (depends on core_gen)
codegen.aski     → codegen_gen (depends on core_gen, parser_gen)
```

## Content-Addressing (criome-store)

Each SemaModule has a content hash (blake3 of its relations).
If the hash matches a stored artifact in criome-store, skip compilation.

```
SemaModule {
    hash: Blake3Hash,        // content-addressed identity
    name: ModuleName,
    exports: Vec<ExportId>,
    imports: Vec<(ModuleName, Blake3Hash)>,  // dep name + expected hash
    types: Vec<SemaType>,
    variants: Vec<SemaVariant>,
    fields: Vec<SemaField>,
    traits: Vec<SemaTraitDecl>,
    impls: Vec<SemaTraitImpl>,
    methods: Vec<SemaMethod>,
    ffi: Vec<SemaFfi>,
    constants: Vec<SemaConst>,
}
```

The hash covers everything EXCEPT the imports' hashes (which are
references, not content). This means a module's hash changes only
when ITS content changes, not when a dependency changes.

But the compiled artifact (Rust crate) depends on the dependency
hashes too — if a dependency's API changes, dependents must recompile
even though their source didn't change. So the compilation cache key
is: `(module_hash, dep_hashes...)`.

## Incremental Compilation Flow

```
1. Parse all .aski files → AskiWorld (one per file, with module data)
2. Lower each → SemaModule (with content hash)
3. For each SemaModule:
   a. Compute compilation key: (module_hash, dep_hashes)
   b. Check criome-store for cached artifact
   c. If cached → skip
   d. If not → codegen to Rust crate → compile → store in criome
4. Link all crates together
```

## What Changes in SemaWorld

Currently SemaWorld is one flat struct. It needs to become a
collection of SemaModules:

```
SemaWorld {
    modules: Vec<SemaModule>,
    // Global name tables (merged from all modules)
    type_names: Vec<String>,
    variant_names: Vec<String>,
    ...
}
```

Each SemaModule owns its relations. The global name tables are
for cross-module resolution during codegen.

## What Changes in AskiWorld

AskiWorld already tracks module data (current_file, module_name,
exports, imports). It needs to:

1. Lower per-module (one SemaModule per parsed file)
2. Track the file → module mapping
3. Resolve cross-module references during parsing (imports)

## Rust Crate Structure

Each SemaModule generates:

```
// core_gen/src/lib.rs
pub enum Element { Fire, Earth, Air, Water }
pub enum Quality { Passionate, Grounded, Intellectual, Intuitive }
pub trait Describe { fn describe(&self) -> Quality; }
impl Describe for Element { ... }
```

```
// parser_gen/src/lib.rs
use core_gen::{Element, Quality};
pub struct Parser { ... }
```

The crate name is derived from the module name: `to_snake(module_name) + "_gen"`.

## .main Files

A .main file generates a binary crate:

```
// hello/src/main.rs
use core_gen::{Element, Quality, Describe};
fn main() {
    let e = Element::Fire;
    println!("{:?}", e.describe());
}
```

## Questions

1. Should each SemaModule be independently rkyv-serializable?
   (Yes — that's what goes into criome-store.)

2. Should the name tables be per-module or global?
   (Per-module for storage, merged for codegen.)

3. How do we handle re-exports? Module A exports something from
   Module B. Does Module A's SemaModule contain a copy or a reference?
   (Reference — SemaModule only has what it declares.)
