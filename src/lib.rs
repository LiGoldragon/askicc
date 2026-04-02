#![allow(non_snake_case)]
//! aski-core — Kernel schema shared between aski-rs and aski-cc.
//!
//! Defines the Ascent World with PascalCase relations (aski naming).
//! This is the contract: aski-rs codegen reads it, aski-cc projects to it.

use std::collections::HashSet;

use ascent::ascent;

// ═══════════════════════════════════════════════════════════════
// Kernel World — the simplified aski AST in relational form
// ═══════════════════════════════════════════════════════════════

ascent! {
    pub struct World;

    // ── Nodes ──
    // Every AST item: domain, struct, trait, impl, method, const, main, type_alias
    relation Node(i64, String, String, Option<i64>, usize, usize);
    // (id, kind, name, parent_id, span_start, span_end)

    // ── Domains ──
    relation Variant(i64, i64, String, Option<String>);
    // (domain_id, ordinal, name, wraps_type)

    // ── Structs ──
    relation Field(i64, i64, String, String);
    // (struct_id, ordinal, name, type_ref)

    // ── Methods ──
    relation Param(i64, i64, String, Option<String>, Option<String>);
    // (method_id, ordinal, kind, name, type_ref)
    // kind: "borrow_self", "mut_borrow_self", "owned_self", "owned", "named", "borrow", "mut_borrow"

    relation Returns(i64, String);
    // (method_id, type_ref)

    // ── Trait system ──
    relation TraitImpl(String, String, i64);
    // (trait_name, type_name, impl_node_id)

    // ── Constants ──
    relation Constant(i64, String, String, bool);
    // (node_id, name, type_ref, has_value)

    // ── Expressions ──
    relation Expr(i64, Option<i64>, String, i64, Option<String>);
    // (id, parent_id, kind, ordinal, value)

    // ── Match arms ──
    relation MatchArm(i64, i64, String, Option<i64>, String);
    // (match_id, ordinal, patterns_json, body_expr_id, arm_kind)

    // ── Derived: type containment ──
    relation ContainedType(String, String);
    // (parent_type, child_type) — immediate containment

    // Auto-derive from struct fields
    ContainedType(parent_type, field_type) <--
        Node(parent_id, kind, parent_type, _, _, _),
        if kind == "struct",
        Field(*parent_id, _, _, field_type);

    // Auto-derive from domain variant wraps
    ContainedType(parent_type, field_type.clone()) <--
        Node(parent_id, kind, parent_type, _, _, _),
        if kind == "domain",
        Variant(*parent_id, _, _, wraps),
        if wraps.is_some(),
        let field_type = wraps.as_ref().unwrap();

    // ── Derived: transitive closure ──
    relation RecursiveType(String, String);
    RecursiveType(x, y) <-- ContainedType(x, y);
    RecursiveType(x, z) <-- ContainedType(x, y), RecursiveType(y, z);
}

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
    world.run();
}

// ═══════════════════════════════════════════════════════════════
// Query functions — plain return types, no Result wrapping
// ═══════════════════════════════════════════════════════════════

/// All top-level nodes ordered by id.
pub fn query_all_top_level_nodes(world: &World) -> Vec<(i64, String, String)> {
    let mut nodes: Vec<_> = world.Node.iter()
        .filter(|(_, _, _, parent, _, _)| parent.is_none())
        .map(|(id, kind, name, _, _, _)| (*id, kind.clone(), name.clone()))
        .collect();
    nodes.sort_by_key(|(id, _, _)| *id);
    nodes
}

/// Child nodes of a parent, ordered by id.
pub fn query_child_nodes(world: &World, parent_id: i64) -> Vec<(i64, String, String)> {
    let mut nodes: Vec<_> = world.Node.iter()
        .filter(|(_, _, _, parent, _, _)| *parent == Some(parent_id))
        .map(|(id, kind, name, _, _, _)| (*id, kind.clone(), name.clone()))
        .collect();
    nodes.sort_by_key(|(id, _, _)| *id);
    nodes
}

/// Domain variants ordered by ordinal.
pub fn query_domain_variants(world: &World, domain_name: &str) -> Vec<(i32, String, Option<String>)> {
    let domain_id = world.Node.iter()
        .find(|(_, kind, name, _, _, _)| kind == "domain" && name == domain_name)
        .map(|(id, _, _, _, _, _)| *id);

    let Some(did) = domain_id else { return Vec::new() };

    let mut variants: Vec<_> = world.Variant.iter()
        .filter(|(id, _, _, _)| *id == did)
        .map(|(_, ordinal, name, wraps)| (*ordinal as i32, name.clone(), wraps.clone()))
        .collect();
    variants.sort_by_key(|(ord, _, _)| *ord);
    variants
}

/// Struct fields ordered by ordinal.
pub fn query_struct_fields(world: &World, struct_name: &str) -> Vec<(i32, String, String)> {
    let struct_id = world.Node.iter()
        .find(|(_, kind, name, _, _, _)| kind == "struct" && name == struct_name)
        .map(|(id, _, _, _, _, _)| *id);

    let Some(sid) = struct_id else { return Vec::new() };

    let mut fields: Vec<_> = world.Field.iter()
        .filter(|(id, _, _, _)| *id == sid)
        .map(|(_, ordinal, name, type_ref)| (*ordinal as i32, name.clone(), type_ref.clone()))
        .collect();
    fields.sort_by_key(|(ord, _, _)| *ord);
    fields
}

/// Parameters ordered by ordinal.
pub fn query_params(world: &World, node_id: i64) -> Vec<(String, Option<String>, Option<String>)> {
    let mut params: Vec<(i64, String, Option<String>, Option<String>)> = world.Param.iter()
        .filter(|(nid, _, _, _, _)| *nid == node_id)
        .map(|(_, ordinal, kind, name, type_ref)| (*ordinal, kind.clone(), name.clone(), type_ref.clone()))
        .collect();
    params.sort_by_key(|(ord, _, _, _)| *ord);
    params.into_iter().map(|(_, kind, name, type_ref)| (kind, name, type_ref)).collect()
}

/// Return type.
pub fn query_return_type(world: &World, node_id: i64) -> Option<String> {
    world.Returns.iter()
        .find(|(nid, _)| *nid == node_id)
        .map(|(_, type_ref)| type_ref.clone())
}

/// Constant definition.
pub fn query_constant(world: &World, node_id: i64) -> Option<(String, String, bool)> {
    world.Constant.iter()
        .find(|(nid, _, _, _)| *nid == node_id)
        .map(|(_, name, type_ref, has_value)| (name.clone(), type_ref.clone(), *has_value))
}

/// Child expressions ordered by ordinal.
pub fn query_child_exprs(world: &World, parent_id: i64) -> Vec<(i64, String, i64, Option<String>)> {
    let mut exprs: Vec<_> = world.Expr.iter()
        .filter(|(_, pid, _, _, _)| *pid == Some(parent_id))
        .map(|(id, _, kind, ordinal, value)| (*id, kind.clone(), *ordinal, value.clone()))
        .collect();
    exprs.sort_by_key(|(_, _, ord, _)| *ord);
    exprs
}

/// Match arms ordered by ordinal.
pub fn query_match_arms(world: &World, match_id: i64) -> Vec<(i64, Vec<String>, Option<i64>, String)> {
    let mut arms: Vec<_> = world.MatchArm.iter()
        .filter(|(mid, _, _, _, _)| *mid == match_id)
        .map(|(_, ordinal, patterns_json, body_id, arm_kind)| {
            let patterns: Vec<String> = serde_json::from_str(patterns_json).unwrap_or_default();
            (*ordinal, patterns, *body_id, arm_kind.clone())
        })
        .collect();
    arms.sort_by_key(|(ord, _, _, _)| *ord);
    arms
}

/// Expression by id.
pub fn query_expr_by_id(world: &World, expr_id: i64) -> Option<(String, Option<String>)> {
    world.Expr.iter()
        .find(|(id, _, _, _, _)| *id == expr_id)
        .map(|(_, _, kind, _, value)| (kind.clone(), value.clone()))
}

/// All nodes of a given kind.
pub fn query_nodes_by_kind(world: &World, kind: &str) -> Vec<(i64, String)> {
    world.Node.iter()
        .filter(|(_, k, _, _, _, _)| k == kind)
        .map(|(id, _, name, _, _, _)| (*id, name.clone()))
        .collect()
}

/// Node kind by id.
pub fn query_node_kind(world: &World, node_id: i64) -> Option<String> {
    world.Node.iter()
        .find(|(id, _, _, _, _, _)| *id == node_id)
        .map(|(_, kind, _, _, _, _)| kind.clone())
}

/// Check if a name is a known method (has a "method" or "tail_method" node).
pub fn is_known_method(name: &str, world: &World) -> bool {
    world.Node.iter().any(|(_, kind, n, _, _, _)| {
        (kind == "method" || kind == "tail_method" || kind == "method_sig") && n == name
    })
}

/// Recursive fields from the derived RecursiveType relation.
pub fn query_recursive_fields(world: &World) -> HashSet<(String, String)> {
    world.RecursiveType.iter()
        .filter(|(parent, child)| parent == child)
        .map(|(parent, _)| {
            // Find the specific field that causes the recursion
            let field = world.Field.iter()
                .find(|(_, _, _, type_ref)| type_ref == parent)
                .map(|(_, _, name, _)| name.clone())
                .unwrap_or_default();
            (parent.clone(), field)
        })
        .collect()
}

/// Validate: body-scoped types should not appear in return positions.
pub fn validate_return_type_scope(world: &World) -> Vec<(String, String)> {
    let mut violations = Vec::new();
    for (node_id, _, name, parent, _, _) in &world.Node {
        if parent.is_some() {
            // This is a body-scoped type — check if it appears in any returns
            for (ret_node, type_ref) in &world.Returns {
                if type_ref == name && ret_node != node_id {
                    violations.push((name.clone(), type_ref.clone()));
                }
            }
        }
    }
    violations
}

/// Validate: String should not appear as a struct field type.
pub fn validate_no_string_fields(world: &World) -> Vec<(String, String)> {
    let mut violations = Vec::new();
    for (struct_id, _, field_name, type_ref) in &world.Field {
        if type_ref == "String" {
            let struct_name = world.Node.iter()
                .find(|(id, _, _, _, _, _)| id == struct_id)
                .map(|(_, _, name, _, _, _)| name.clone())
                .unwrap_or_default();
            violations.push((struct_name, field_name.clone()));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_type_detection() {
        let mut world = World::default();
        world.Node.push((1, "struct".into(), "Tree".into(), None, 0, 0));
        world.Node.push((2, "struct".into(), "Branch".into(), None, 0, 0));
        world.Field.push((1, 0, "children".into(), "Branch".into()));
        world.Field.push((2, 0, "subtree".into(), "Tree".into()));
        run_rules(&mut world);
        // Tree → Branch → Tree is a cycle
        assert!(world.RecursiveType.contains(&("Tree".into(), "Tree".into())));
        assert!(world.RecursiveType.contains(&("Branch".into(), "Branch".into())));
    }

    #[test]
    fn linear_containment_no_cycle() {
        let mut world = World::default();
        world.Node.push((1, "struct".into(), "A".into(), None, 0, 0));
        world.Node.push((2, "struct".into(), "B".into(), None, 0, 0));
        world.Node.push((3, "struct".into(), "C".into(), None, 0, 0));
        world.Field.push((1, 0, "b".into(), "B".into()));
        world.Field.push((2, 0, "c".into(), "C".into()));
        run_rules(&mut world);
        assert!(world.RecursiveType.contains(&("A".into(), "C".into())));
        assert!(!world.RecursiveType.contains(&("C".into(), "A".into())));
    }

    #[test]
    fn query_top_level() {
        let mut world = World::default();
        world.Node.push((1, "domain".into(), "Element".into(), None, 0, 10));
        world.Node.push((2, "struct".into(), "Point".into(), None, 11, 20));
        world.Node.push((3, "method".into(), "distance".into(), Some(2), 21, 30));
        let top = query_all_top_level_nodes(&world);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].2, "Element");
        assert_eq!(top[1].2, "Point");
    }
}
