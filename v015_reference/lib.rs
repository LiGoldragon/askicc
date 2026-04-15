#![allow(non_snake_case)]
//! aski-core — Kernel schema shared between aski-rs and aski-cc.
//!
//! Everything here is generated from kernel.aski by askic.
//! The World struct, relation types, queries, and derivation rules
//! are all generated — no hand-written Ascent.

// World, enums, structs, queries, derive() — all generated from kernel.aski
include!(concat!(env!("OUT_DIR"), "/kernel.rs"));

// ═══════════════════════════════════════════════════════════════
// ID Generator
// ═══════════════════════════════════════════════════════════════

pub struct IdGen {
    pub next: i64,
}

impl IdGen {
    pub fn new() -> Self {
        Self { next: 1 }
    }

    pub fn next(&mut self) -> i64 {
        let id = self.next;
        self.next += 1;
        id
    }
}

impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════
// Pattern utilities
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum ParsedPattern {
    Variant(String),
    Wildcard,
    BoolLit(bool),
    DataCarrying(String, String),
}

pub fn parse_pattern_string(s: &str) -> ParsedPattern {
    match s {
        "_" => ParsedPattern::Wildcard,
        "True" => ParsedPattern::BoolLit(true),
        "False" => ParsedPattern::BoolLit(false),
        other => {
            if let Some(paren_pos) = other.find('(') {
                if other.ends_with(')') {
                    let name = other[..paren_pos].to_string();
                    let inner = other[paren_pos + 1..other.len() - 1].to_string();
                    return ParsedPattern::DataCarrying(name, inner);
                }
            }
            ParsedPattern::Variant(other.to_string())
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// World lifecycle
// ═══════════════════════════════════════════════════════════════

pub fn run_rules(world: &mut World) {
    world.derive();
}

// ═══════════════════════════════════════════════════════════════
// Compatibility query functions
// These wrap the generated World methods to match the old API
// that aski-rs codegen and ir.rs expect.
// ═══════════════════════════════════════════════════════════════

/// All top-level nodes (parent == 0) ordered by id.
pub fn query_all_top_level_nodes(world: &World) -> Vec<(i64, String, String)> {
    let mut nodes: Vec<_> = world.nodes.iter()
        .filter(|n| n.parent == 0)
        .map(|n| (n.id, n.kind.to_str().to_string(), n.name.clone()))
        .collect();
    nodes.sort_by_key(|(id, _, _)| *id);
    nodes
}

/// Child nodes of a parent, ordered by id.
pub fn query_child_nodes(world: &World, parent_id: i64) -> Vec<(i64, String, String)> {
    let mut nodes: Vec<_> = world.nodes.iter()
        .filter(|n| n.parent == parent_id && parent_id != 0)
        .map(|n| (n.id, n.kind.to_str().to_string(), n.name.clone()))
        .collect();
    nodes.sort_by_key(|(id, _, _)| *id);
    nodes
}

/// Domain variants ordered by ordinal.
pub fn query_domain_variants(world: &World, domain_name: &str) -> Vec<(i32, String, Option<String>)> {
    let domain_id = world.nodes.iter()
        .find(|n| n.kind == NodeKind::Domain && n.name == domain_name)
        .map(|n| n.id);

    let Some(did) = domain_id else { return Vec::new() };

    let mut variants: Vec<_> = world.variants.iter()
        .filter(|v| v.domain_id == did)
        .map(|v| (v.ordinal as i32, v.name.clone(), if v.wraps_type.is_empty() { None } else { Some(v.wraps_type.clone()) }))
        .collect();
    variants.sort_by_key(|(ord, _, _)| *ord);
    variants
}

/// Struct fields ordered by ordinal.
pub fn query_struct_fields(world: &World, struct_name: &str) -> Vec<(i32, String, String)> {
    let struct_id = world.nodes.iter()
        .find(|n| n.kind == NodeKind::Struct && n.name == struct_name)
        .map(|n| n.id);

    let Some(sid) = struct_id else { return Vec::new() };

    let mut fields: Vec<_> = world.fields.iter()
        .filter(|f| f.struct_id == sid)
        .map(|f| (f.ordinal as i32, f.name.clone(), f.type_ref.clone()))
        .collect();
    fields.sort_by_key(|(ord, _, _)| *ord);
    fields
}

/// Parameters ordered by ordinal.
pub fn query_params(world: &World, node_id: i64) -> Vec<(String, Option<String>, Option<String>)> {
    let mut params: Vec<(i64, String, Option<String>, Option<String>)> = world.params.iter()
        .filter(|p| p.method_id == node_id)
        .map(|p| (p.ordinal, p.kind.to_str().to_string(),
                   if p.name.is_empty() { None } else { Some(p.name.clone()) },
                   if p.type_ref.is_empty() { None } else { Some(p.type_ref.clone()) }))
        .collect();
    params.sort_by_key(|(ord, _, _, _)| *ord);
    params.into_iter().map(|(_, kind, name, type_ref)| (kind, name, type_ref)).collect()
}

/// Return type.
pub fn query_return_type(world: &World, node_id: i64) -> Option<String> {
    world.returns.iter()
        .find(|r| r.method_id == node_id)
        .map(|r| r.type_ref.clone())
}

/// Constant definition.
pub fn query_constant(world: &World, node_id: i64) -> Option<(String, String, bool)> {
    world.constants.iter()
        .find(|c| c.node_id == node_id)
        .map(|c| (c.name.clone(), c.type_ref.clone(), c.has_value))
}

/// Child expressions ordered by ordinal.
pub fn query_child_exprs(world: &World, parent_id: i64) -> Vec<(i64, String, i64, Option<String>)> {
    let mut exprs: Vec<_> = world.exprs.iter()
        .filter(|e| e.parent_id == parent_id && parent_id != 0)
        .map(|e| (e.id, e.kind.to_str().to_string(), e.ordinal,
                   if e.value.is_empty() { None } else { Some(e.value.clone()) }))
        .collect();
    exprs.sort_by_key(|(_, _, ord, _)| *ord);
    exprs
}

/// Match arms ordered by ordinal.
pub fn query_match_arms(world: &World, match_id: i64) -> Vec<(i64, Vec<String>, Option<i64>, String)> {
    let mut arms: Vec<_> = world.match_arms.iter()
        .filter(|a| a.match_id == match_id)
        .map(|a| {
            let mut patterns: Vec<_> = world.match_patterns.iter()
                .filter(|p| p.match_id == match_id && p.arm_ordinal == a.ordinal)
                .collect::<Vec<_>>();
            patterns.sort_by_key(|p| p.pat_ordinal);
            let pat_strs: Vec<String> = patterns.iter().map(|p| p.value.clone()).collect();
            let body_id = if a.body_expr_id == 0 { None } else { Some(a.body_expr_id) };
            (a.ordinal, pat_strs, body_id, a.kind.to_str().to_string())
        })
        .collect();
    arms.sort_by_key(|(ord, _, _, _)| *ord);
    arms
}

/// Expression by id.
pub fn query_expr_by_id(world: &World, expr_id: i64) -> Option<(String, Option<String>)> {
    world.exprs.iter()
        .find(|e| e.id == expr_id)
        .map(|e| (e.kind.to_str().to_string(),
                   if e.value.is_empty() { None } else { Some(e.value.clone()) }))
}

/// All nodes of a given kind.
pub fn query_nodes_by_kind(world: &World, kind: &str) -> Vec<(i64, String)> {
    let k = NodeKind::from_str(kind);
    world.nodes.iter()
        .filter(|n| k.map_or(false, |k| n.kind == k))
        .map(|n| (n.id, n.name.clone()))
        .collect()
}

/// Node kind by id.
pub fn query_node_kind(world: &World, node_id: i64) -> Option<String> {
    world.nodes.iter()
        .find(|n| n.id == node_id)
        .map(|n| n.kind.to_str().to_string())
}

/// Check if a name is a known method.
pub fn is_known_method(name: &str, world: &World) -> bool {
    world.nodes.iter().any(|n| {
        (n.kind == NodeKind::Method || n.kind == NodeKind::TailMethod || n.kind == NodeKind::MethodSig)
            && n.name == name
    })
}

/// Recursive fields from the derived RecursiveType relation.
pub fn query_recursive_fields(world: &World) -> std::collections::HashSet<(String, String)> {
    world.recursive_types.iter()
        .filter(|r| r.parent_type == r.child_type)
        .map(|r| {
            let field = world.fields.iter()
                .find(|f| f.type_ref == r.parent_type)
                .map(|f| f.name.clone())
                .unwrap_or_default();
            (r.parent_type.clone(), field)
        })
        .collect()
}

/// Validate: body-scoped types should not appear in return positions.
pub fn validate_return_type_scope(world: &World) -> Vec<(String, String)> {
    let mut violations = Vec::new();
    for node in &world.nodes {
        if node.parent != 0 {
            for ret in &world.returns {
                if ret.type_ref == node.name && ret.method_id != node.id {
                    violations.push((node.name.clone(), ret.type_ref.clone()));
                }
            }
        }
    }
    violations
}

/// Supertraits of a trait.
pub fn query_supertraits(world: &World, trait_id: i64) -> Vec<String> {
    world.supertraits.iter()
        .filter(|s| s.trait_node_id == trait_id)
        .map(|s| s.supertrait_name.clone())
        .collect()
}

/// Validate: String should not appear as a struct field type.
pub fn validate_no_string_fields(world: &World) -> Vec<(String, String)> {
    let mut violations = Vec::new();
    for field in &world.fields {
        if field.type_ref == "String" {
            let struct_name = world.nodes.iter()
                .find(|n| n.id == field.struct_id)
                .map(|n| n.name.clone())
                .unwrap_or_default();
            violations.push((struct_name, field.name.clone()));
        }
    }
    violations
}

/// Which domain owns this variant name?
pub fn query_variant_domain(world: &World, variant_name: &str) -> Option<(String, i64)> {
    world.variant_ofs.iter()
        .find(|v| v.variant_name == variant_name)
        .map(|v| (v.domain_name.clone(), v.domain_node_id))
}

/// Binding info for an expression.
pub fn query_binding_info(world: &World, expr_id: i64) -> Option<(String, String)> {
    world.binding_infos.iter()
        .find(|b| b.expr_id == expr_id)
        .map(|b| (b.var_name.clone(), b.type_name.clone()))
}

/// What kind of type is this name?
pub fn query_type_kind(world: &World, name: &str) -> Option<String> {
    world.type_kinds.iter()
        .find(|t| t.type_name == name)
        .map(|t| t.category.to_str().to_string())
}

/// What methods are available on a type via trait impls?
pub fn query_methods_on_type(world: &World, type_name: &str) -> Vec<(String, i64)> {
    world.method_on_types.iter()
        .filter(|m| m.type_name == type_name)
        .map(|m| (m.method_name.clone(), m.method_node_id))
        .collect()
}

/// Is this trait impl an operator trait? Returns (rust_import, output_type).
pub fn query_operator_impl(world: &World, trait_name: &str, type_name: &str) -> Option<(String, String)> {
    world.operator_impls.iter()
        .find(|o| o.trait_name == trait_name && o.type_name == type_name)
        .map(|o| (o.rust_import.clone(), o.output_type.clone()))
}

/// All operator trait impls.
pub fn query_all_operator_impls(world: &World) -> Vec<(String, String, String)> {
    world.operator_impls.iter()
        .map(|o| (o.trait_name.clone(), o.type_name.clone(), o.rust_import.clone()))
        .collect()
}

/// Is this field recursive (needs Box<T>)?
pub fn is_recursive_field(world: &World, struct_name: &str, field_name: &str) -> bool {
    world.recursive_fields.iter()
        .any(|r| r.struct_name == struct_name && r.field_name == field_name)
}

/// Mutable binding info for an expression.
pub fn query_mutable_binding(world: &World, expr_id: i64) -> Option<(String, String)> {
    world.mutable_bindings.iter()
        .find(|m| m.expr_id == expr_id)
        .map(|m| (m.var_name.clone(), m.type_name.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: i64, kind: NodeKind, name: &str, parent: i64, scope_id: i64) -> Node {
        Node { id, kind, name: name.to_string(), parent, span_start: 0, span_end: 0, scope_id }
    }

    #[test]
    fn recursive_type_detection() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Struct, "Tree", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Struct, "Branch", 0, 0));
        world.fields.push(Field { struct_id: 1, ordinal: 0, name: "children".into(), type_ref: "Branch".into() });
        world.fields.push(Field { struct_id: 2, ordinal: 0, name: "subtree".into(), type_ref: "Tree".into() });
        run_rules(&mut world);
        assert!(world.recursive_types.iter().any(|r| r.parent_type == "Tree" && r.child_type == "Tree"));
        assert!(world.recursive_types.iter().any(|r| r.parent_type == "Branch" && r.child_type == "Branch"));
    }

    #[test]
    fn linear_containment_no_cycle() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Struct, "A", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Struct, "B", 0, 0));
        world.nodes.push(make_node(3, NodeKind::Struct, "C", 0, 0));
        world.fields.push(Field { struct_id: 1, ordinal: 0, name: "b".into(), type_ref: "B".into() });
        world.fields.push(Field { struct_id: 2, ordinal: 0, name: "c".into(), type_ref: "C".into() });
        run_rules(&mut world);
        assert!(world.recursive_types.iter().any(|r| r.parent_type == "A" && r.child_type == "C"));
        assert!(!world.recursive_types.iter().any(|r| r.parent_type == "C" && r.child_type == "A"));
    }

    #[test]
    fn query_top_level() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Domain, "Element", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Struct, "Point", 0, 0));
        world.nodes.push(make_node(3, NodeKind::Method, "distance", 2, 0));
        let top = query_all_top_level_nodes(&world);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].2, "Element");
        assert_eq!(top[1].2, "Point");
    }

    #[test]
    fn qualified_name_no_scope() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Domain, "Element", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Struct, "Color", 0, 0));
        run_rules(&mut world);
        assert!(world.qualified_names.iter().any(|q| q.node_id == 1 && q.full_path == "Element"));
        assert!(world.qualified_names.iter().any(|q| q.node_id == 2 && q.full_path == "Color"));
    }

    #[test]
    fn qualified_name_with_scope() {
        let mut world = World::default();
        world.scopes.push(Scope { id: 100, kind: ScopeKind::Module, name: "astro".into(), parent_scope_id: 0 });
        world.nodes.push(make_node(1, NodeKind::Domain, "Sign", 0, 100));
        world.nodes.push(make_node(2, NodeKind::Domain, "Planet", 0, 100));
        run_rules(&mut world);
        assert!(world.qualified_names.iter().any(|q| q.node_id == 1 && q.full_path == "astro::Sign"));
        assert!(world.qualified_names.iter().any(|q| q.node_id == 2 && q.full_path == "astro::Planet"));
    }

    #[test]
    fn qualified_name_child_nodes() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Struct, "Point", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Method, "distance", 1, 0));
        run_rules(&mut world);
        assert!(world.qualified_names.iter().any(|q| q.node_id == 1 && q.full_path == "Point"));
        assert!(world.qualified_names.iter().any(|q| q.node_id == 2 && q.full_path == "Point::distance"));
    }

    #[test]
    fn can_see_self_and_siblings() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Domain, "Element", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Domain, "Sign", 0, 0));
        run_rules(&mut world);
        assert!(world.can_sees.iter().any(|c| c.observer_id == 1 && c.visible_id == 1));
        assert!(world.can_sees.iter().any(|c| c.observer_id == 2 && c.visible_id == 2));
        assert!(world.can_sees.iter().any(|c| c.observer_id == 1 && c.visible_id == 2));
        assert!(world.can_sees.iter().any(|c| c.observer_id == 2 && c.visible_id == 1));
    }

    #[test]
    fn can_see_inherited_from_parent() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Domain, "Element", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Struct, "Color", 0, 0));
        world.nodes.push(make_node(3, NodeKind::Method, "bright", 2, 0));
        run_rules(&mut world);
        assert!(world.can_sees.iter().any(|c| c.observer_id == 2 && c.visible_id == 1));
        assert!(world.can_sees.iter().any(|c| c.observer_id == 3 && c.visible_id == 1));
        assert!(world.can_sees.iter().any(|c| c.observer_id == 3 && c.visible_id == 2));
    }

    #[test]
    fn rules_required_for_derived_relations() {
        let mut world = World::default();
        world.scopes.push(Scope { id: 100, kind: ScopeKind::Module, name: "astro".into(), parent_scope_id: 0 });
        world.nodes.push(make_node(1, NodeKind::Domain, "Sign", 0, 100));
        world.nodes.push(make_node(2, NodeKind::Domain, "Planet", 0, 100));
        assert!(world.qualified_names.is_empty());
        assert!(world.can_sees.is_empty());
        run_rules(&mut world);
        assert!(!world.qualified_names.is_empty());
        assert!(!world.can_sees.is_empty());
    }

    #[test]
    fn visibility_does_not_leak_downward() {
        let mut world = World::default();
        world.nodes.push(make_node(1, NodeKind::Domain, "Element", 0, 0));
        world.nodes.push(make_node(2, NodeKind::Struct, "Color", 0, 0));
        world.nodes.push(make_node(3, NodeKind::Method, "bright", 2, 0));
        world.nodes.push(make_node(4, NodeKind::Struct, "Intermediate", 3, 0));
        run_rules(&mut world);
        assert!(!world.can_sees.iter().any(|c| c.observer_id == 1 && c.visible_id == 4));
    }

    #[test]
    fn can_see_through_imports() {
        let mut world = World::default();
        world.scopes.push(Scope { id: 100, kind: ScopeKind::Module, name: "chart".into(), parent_scope_id: 0 });
        world.scopes.push(Scope { id: 200, kind: ScopeKind::Module, name: "render".into(), parent_scope_id: 0 });
        world.nodes.push(make_node(1, NodeKind::Domain, "Element", 0, 100));
        world.exports.push(Export { scope_id: 100, exported_name: "Element".into() });
        world.nodes.push(make_node(2, NodeKind::Struct, "Color", 0, 200));
        world.imports.push(Import { scope_id: 200, source_module: "chart".into(), imported_name: "Element".into() });
        run_rules(&mut world);
        assert!(world.can_sees.iter().any(|c| c.observer_id == 2 && c.visible_id == 1));
    }
}
