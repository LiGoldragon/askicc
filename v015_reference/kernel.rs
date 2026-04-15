#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Domain,
    Struct,
    Trait,
    Impl,
    ImplBody,
    Method,
    TailMethod,
    MethodSig,
    Const,
    Main,
    TypeAlias,
    GrammarRuleNode,
    ForeignBlock,
    ForeignFunction,
    AssocType,
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl NodeKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "domain" => Some(Self::Domain),
            "struct" => Some(Self::Struct),
            "trait" => Some(Self::Trait),
            "impl" => Some(Self::Impl),
            "impl_body" => Some(Self::ImplBody),
            "ImplBody" => Some(Self::ImplBody),
            "method" => Some(Self::Method),
            "tail_method" => Some(Self::TailMethod),
            "TailMethod" => Some(Self::TailMethod),
            "method_sig" => Some(Self::MethodSig),
            "MethodSig" => Some(Self::MethodSig),
            "const" => Some(Self::Const),
            "main" => Some(Self::Main),
            "type_alias" => Some(Self::TypeAlias),
            "TypeAlias" => Some(Self::TypeAlias),
            "grammar_rule_node" => Some(Self::GrammarRuleNode),
            "GrammarRuleNode" => Some(Self::GrammarRuleNode),
            "foreign_block" => Some(Self::ForeignBlock),
            "ForeignBlock" => Some(Self::ForeignBlock),
            "foreign_function" => Some(Self::ForeignFunction),
            "ForeignFunction" => Some(Self::ForeignFunction),
            "assoc_type" => Some(Self::AssocType),
            "AssocType" => Some(Self::AssocType),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Struct => "struct",
            Self::Trait => "trait",
            Self::Impl => "impl",
            Self::ImplBody => "impl_body",
            Self::Method => "method",
            Self::TailMethod => "tail_method",
            Self::MethodSig => "method_sig",
            Self::Const => "const",
            Self::Main => "main",
            Self::TypeAlias => "type_alias",
            Self::GrammarRuleNode => "grammar_rule_node",
            Self::ForeignBlock => "foreign_block",
            Self::ForeignFunction => "foreign_function",
            Self::AssocType => "assoc_type",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamKind {
    BorrowSelf,
    MutBorrowSelf,
    OwnedSelf,
    Owned,
    Named,
    Borrow,
    MutBorrow,
}

impl std::fmt::Display for ParamKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ParamKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "borrow_self" => Some(Self::BorrowSelf),
            "BorrowSelf" => Some(Self::BorrowSelf),
            "mut_borrow_self" => Some(Self::MutBorrowSelf),
            "MutBorrowSelf" => Some(Self::MutBorrowSelf),
            "owned_self" => Some(Self::OwnedSelf),
            "OwnedSelf" => Some(Self::OwnedSelf),
            "owned" => Some(Self::Owned),
            "named" => Some(Self::Named),
            "borrow" => Some(Self::Borrow),
            "mut_borrow" => Some(Self::MutBorrow),
            "MutBorrow" => Some(Self::MutBorrow),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::BorrowSelf => "borrow_self",
            Self::MutBorrowSelf => "mut_borrow_self",
            Self::OwnedSelf => "owned_self",
            Self::Owned => "owned",
            Self::Named => "named",
            Self::Borrow => "borrow",
            Self::MutBorrow => "mut_borrow",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmKind {
    Commit,
    Backtrack,
    Destructure,
}

impl std::fmt::Display for ArmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ArmKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "commit" => Some(Self::Commit),
            "backtrack" => Some(Self::Backtrack),
            "destructure" => Some(Self::Destructure),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Commit => "commit",
            Self::Backtrack => "backtrack",
            Self::Destructure => "destructure",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Trait,
    Method,
    Block,
}

impl std::fmt::Display for ScopeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ScopeKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "module" => Some(Self::Module),
            "trait" => Some(Self::Trait),
            "method" => Some(Self::Method),
            "block" => Some(Self::Block),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Trait => "trait",
            Self::Method => "method",
            Self::Block => "block",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExprKind {
    IntLit,
    FloatLit,
    StringLit,
    ConstRef,
    InstanceRef,
    BareName,
    Return,
    Stub,
    Match,
    ErrorProp,
    Yield,
    BinOp,
    Group,
    InlineEval,
    FnCall,
    MethodCall,
    SameTypeNew,
    SubTypeNew,
    MutableNew,
    MutableSet,
    SubTypeDecl,
    DeferredNew,
    StructConstruct,
    StructField,
    Access,
    RangeExclusive,
    RangeInclusive,
    StdOut,
    ExternName,
}

impl std::fmt::Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ExprKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "int_lit" => Some(Self::IntLit),
            "IntLit" => Some(Self::IntLit),
            "float_lit" => Some(Self::FloatLit),
            "FloatLit" => Some(Self::FloatLit),
            "string_lit" => Some(Self::StringLit),
            "StringLit" => Some(Self::StringLit),
            "const_ref" => Some(Self::ConstRef),
            "ConstRef" => Some(Self::ConstRef),
            "instance_ref" => Some(Self::InstanceRef),
            "InstanceRef" => Some(Self::InstanceRef),
            "bare_name" => Some(Self::BareName),
            "BareName" => Some(Self::BareName),
            "return" => Some(Self::Return),
            "stub" => Some(Self::Stub),
            "match" => Some(Self::Match),
            "error_prop" => Some(Self::ErrorProp),
            "ErrorProp" => Some(Self::ErrorProp),
            "yield" => Some(Self::Yield),
            "bin_op" => Some(Self::BinOp),
            "BinOp" => Some(Self::BinOp),
            "group" => Some(Self::Group),
            "inline_eval" => Some(Self::InlineEval),
            "InlineEval" => Some(Self::InlineEval),
            "fn_call" => Some(Self::FnCall),
            "FnCall" => Some(Self::FnCall),
            "method_call" => Some(Self::MethodCall),
            "MethodCall" => Some(Self::MethodCall),
            "same_type_new" => Some(Self::SameTypeNew),
            "SameTypeNew" => Some(Self::SameTypeNew),
            "sub_type_new" => Some(Self::SubTypeNew),
            "SubTypeNew" => Some(Self::SubTypeNew),
            "mutable_new" => Some(Self::MutableNew),
            "MutableNew" => Some(Self::MutableNew),
            "mutable_set" => Some(Self::MutableSet),
            "MutableSet" => Some(Self::MutableSet),
            "sub_type_decl" => Some(Self::SubTypeDecl),
            "SubTypeDecl" => Some(Self::SubTypeDecl),
            "deferred_new" => Some(Self::DeferredNew),
            "DeferredNew" => Some(Self::DeferredNew),
            "struct_construct" => Some(Self::StructConstruct),
            "StructConstruct" => Some(Self::StructConstruct),
            "struct_field" => Some(Self::StructField),
            "StructField" => Some(Self::StructField),
            "access" => Some(Self::Access),
            "range_exclusive" => Some(Self::RangeExclusive),
            "RangeExclusive" => Some(Self::RangeExclusive),
            "range_inclusive" => Some(Self::RangeInclusive),
            "RangeInclusive" => Some(Self::RangeInclusive),
            "std_out" => Some(Self::StdOut),
            "StdOut" => Some(Self::StdOut),
            "extern_name" => Some(Self::ExternName),
            "ExternName" => Some(Self::ExternName),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::IntLit => "int_lit",
            Self::FloatLit => "float_lit",
            Self::StringLit => "string_lit",
            Self::ConstRef => "const_ref",
            Self::InstanceRef => "instance_ref",
            Self::BareName => "bare_name",
            Self::Return => "return",
            Self::Stub => "stub",
            Self::Match => "match",
            Self::ErrorProp => "error_prop",
            Self::Yield => "yield",
            Self::BinOp => "bin_op",
            Self::Group => "group",
            Self::InlineEval => "inline_eval",
            Self::FnCall => "fn_call",
            Self::MethodCall => "method_call",
            Self::SameTypeNew => "same_type_new",
            Self::SubTypeNew => "sub_type_new",
            Self::MutableNew => "mutable_new",
            Self::MutableSet => "mutable_set",
            Self::SubTypeDecl => "sub_type_decl",
            Self::DeferredNew => "deferred_new",
            Self::StructConstruct => "struct_construct",
            Self::StructField => "struct_field",
            Self::Access => "access",
            Self::RangeExclusive => "range_exclusive",
            Self::RangeInclusive => "range_inclusive",
            Self::StdOut => "std_out",
            Self::ExternName => "extern_name",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCategory {
    Domain,
    Struct,
    Primitive,
}

impl std::fmt::Display for TypeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TypeCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "domain" => Some(Self::Domain),
            "struct" => Some(Self::Struct),
            "primitive" => Some(Self::Primitive),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Struct => "struct",
            Self::Primitive => "primitive",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: i64,
    pub kind: NodeKind,
    pub name: String,
    pub parent: i64,
    pub span_start: i64,
    pub span_end: i64,
    pub scope_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub domain_id: i64,
    pub ordinal: i64,
    pub name: String,
    pub wraps_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub struct_id: i64,
    pub ordinal: i64,
    pub name: String,
    pub type_ref: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub method_id: i64,
    pub ordinal: i64,
    pub kind: ParamKind,
    pub name: String,
    pub type_ref: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Returns {
    pub method_id: i64,
    pub type_ref: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub id: i64,
    pub kind: ScopeKind,
    pub name: String,
    pub parent_scope_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Supertrait {
    pub trait_node_id: i64,
    pub supertrait_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    pub scope_id: i64,
    pub exported_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub scope_id: i64,
    pub source_module: String,
    pub imported_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitImpl {
    pub trait_name: String,
    pub type_name: String,
    pub impl_node_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub node_id: i64,
    pub name: String,
    pub type_ref: String,
    pub has_value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub id: i64,
    pub parent_id: i64,
    pub kind: ExprKind,
    pub ordinal: i64,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub match_id: i64,
    pub ordinal: i64,
    pub body_expr_id: i64,
    pub kind: ArmKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPattern {
    pub match_id: i64,
    pub arm_ordinal: i64,
    pub pat_ordinal: i64,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarRule {
    pub node_id: i64,
    pub rule_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarArm {
    pub rule_id: i64,
    pub ordinal: i64,
    pub result_expr_id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatElemKind {
    Terminal,
    NonTerminal,
    Binding,
    BindLit,
    Rest,
}

impl std::fmt::Display for PatElemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PatElemKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "terminal" => Some(Self::Terminal),
            "Terminal" => Some(Self::Terminal),
            "non_terminal" => Some(Self::NonTerminal),
            "NonTerminal" => Some(Self::NonTerminal),
            "binding" => Some(Self::Binding),
            "Binding" => Some(Self::Binding),
            "bind_lit" => Some(Self::BindLit),
            "BindLit" => Some(Self::BindLit),
            "rest" => Some(Self::Rest),
            "Rest" => Some(Self::Rest),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::NonTerminal => "non_terminal",
            Self::Binding => "binding",
            Self::BindLit => "bind_lit",
            Self::Rest => "rest",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarPatElem {
    pub rule_id: i64,
    pub arm_ordinal: i64,
    pub elem_ordinal: i64,
    pub kind: PatElemKind,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedName {
    pub node_id: i64,
    pub full_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanSee {
    pub observer_id: i64,
    pub visible_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantOf {
    pub variant_name: String,
    pub domain_name: String,
    pub domain_node_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindingInfo {
    pub expr_id: i64,
    pub var_name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeKind {
    pub type_name: String,
    pub category: TypeCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodOnType {
    pub type_name: String,
    pub method_name: String,
    pub method_node_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainedType {
    pub parent_type: String,
    pub child_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveType {
    pub parent_type: String,
    pub child_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperatorImpl {
    pub trait_name: String,
    pub type_name: String,
    pub rust_import: String,
    pub output_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveField {
    pub struct_name: String,
    pub field_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MutableBinding {
    pub expr_id: i64,
    pub var_name: String,
    pub type_name: String,
}

/// Kernel World — holds all relations as Vec<T>.
#[derive(Debug, Clone, Default)]
pub struct World {
    pub nodes: Vec<Node>,
    pub variants: Vec<Variant>,
    pub fields: Vec<Field>,
    pub params: Vec<Param>,
    pub returns: Vec<Returns>,
    pub scopes: Vec<Scope>,
    pub supertraits: Vec<Supertrait>,
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    pub trait_impls: Vec<TraitImpl>,
    pub constants: Vec<Constant>,
    pub exprs: Vec<Expr>,
    pub match_arms: Vec<MatchArm>,
    pub match_patterns: Vec<MatchPattern>,
    pub grammar_rules: Vec<GrammarRule>,
    pub grammar_arms: Vec<GrammarArm>,
    pub grammar_pat_elems: Vec<GrammarPatElem>,
    pub qualified_names: Vec<QualifiedName>,
    pub can_sees: Vec<CanSee>,
    pub variant_ofs: Vec<VariantOf>,
    pub binding_infos: Vec<BindingInfo>,
    pub type_kinds: Vec<TypeKind>,
    pub method_on_types: Vec<MethodOnType>,
    pub contained_types: Vec<ContainedType>,
    pub recursive_types: Vec<RecursiveType>,
    pub operator_impls: Vec<OperatorImpl>,
    pub recursive_fields: Vec<RecursiveField>,
    pub mutable_bindings: Vec<MutableBinding>,
}

impl World {
    pub fn new() -> Self { Self::default() }

    pub fn node_by_id(&self, val: i64) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.id == val).collect()
    }

    pub fn node_by_kind(&self, val: NodeKind) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.kind == val).collect()
    }

    pub fn node_by_name(&self, val: &str) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.name == val).collect()
    }

    pub fn node_by_parent(&self, val: i64) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.parent == val).collect()
    }

    pub fn node_by_span_start(&self, val: i64) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.span_start == val).collect()
    }

    pub fn node_by_span_end(&self, val: i64) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.span_end == val).collect()
    }

    pub fn node_by_scope_id(&self, val: i64) -> Vec<&Node> {
        self.nodes.iter().filter(|r| r.scope_id == val).collect()
    }

    pub fn variant_by_domain_id(&self, val: i64) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.domain_id == val).collect()
    }

    pub fn variant_by_ordinal(&self, val: i64) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn variant_by_name(&self, val: &str) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.name == val).collect()
    }

    pub fn variant_by_wraps_type(&self, val: &str) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.wraps_type == val).collect()
    }

    pub fn field_by_struct_id(&self, val: i64) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.struct_id == val).collect()
    }

    pub fn field_by_ordinal(&self, val: i64) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn field_by_name(&self, val: &str) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.name == val).collect()
    }

    pub fn field_by_type_ref(&self, val: &str) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.type_ref == val).collect()
    }

    pub fn param_by_method_id(&self, val: i64) -> Vec<&Param> {
        self.params.iter().filter(|r| r.method_id == val).collect()
    }

    pub fn param_by_ordinal(&self, val: i64) -> Vec<&Param> {
        self.params.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn param_by_kind(&self, val: ParamKind) -> Vec<&Param> {
        self.params.iter().filter(|r| r.kind == val).collect()
    }

    pub fn param_by_name(&self, val: &str) -> Vec<&Param> {
        self.params.iter().filter(|r| r.name == val).collect()
    }

    pub fn param_by_type_ref(&self, val: &str) -> Vec<&Param> {
        self.params.iter().filter(|r| r.type_ref == val).collect()
    }

    pub fn returns_by_method_id(&self, val: i64) -> Vec<&Returns> {
        self.returns.iter().filter(|r| r.method_id == val).collect()
    }

    pub fn returns_by_type_ref(&self, val: &str) -> Vec<&Returns> {
        self.returns.iter().filter(|r| r.type_ref == val).collect()
    }

    pub fn scope_by_id(&self, val: i64) -> Vec<&Scope> {
        self.scopes.iter().filter(|r| r.id == val).collect()
    }

    pub fn scope_by_kind(&self, val: ScopeKind) -> Vec<&Scope> {
        self.scopes.iter().filter(|r| r.kind == val).collect()
    }

    pub fn scope_by_name(&self, val: &str) -> Vec<&Scope> {
        self.scopes.iter().filter(|r| r.name == val).collect()
    }

    pub fn scope_by_parent_scope_id(&self, val: i64) -> Vec<&Scope> {
        self.scopes.iter().filter(|r| r.parent_scope_id == val).collect()
    }

    pub fn supertrait_by_trait_node_id(&self, val: i64) -> Vec<&Supertrait> {
        self.supertraits.iter().filter(|r| r.trait_node_id == val).collect()
    }

    pub fn supertrait_by_supertrait_name(&self, val: &str) -> Vec<&Supertrait> {
        self.supertraits.iter().filter(|r| r.supertrait_name == val).collect()
    }

    pub fn export_by_scope_id(&self, val: i64) -> Vec<&Export> {
        self.exports.iter().filter(|r| r.scope_id == val).collect()
    }

    pub fn export_by_exported_name(&self, val: &str) -> Vec<&Export> {
        self.exports.iter().filter(|r| r.exported_name == val).collect()
    }

    pub fn import_by_scope_id(&self, val: i64) -> Vec<&Import> {
        self.imports.iter().filter(|r| r.scope_id == val).collect()
    }

    pub fn import_by_source_module(&self, val: &str) -> Vec<&Import> {
        self.imports.iter().filter(|r| r.source_module == val).collect()
    }

    pub fn import_by_imported_name(&self, val: &str) -> Vec<&Import> {
        self.imports.iter().filter(|r| r.imported_name == val).collect()
    }

    pub fn trait_impl_by_trait_name(&self, val: &str) -> Vec<&TraitImpl> {
        self.trait_impls.iter().filter(|r| r.trait_name == val).collect()
    }

    pub fn trait_impl_by_type_name(&self, val: &str) -> Vec<&TraitImpl> {
        self.trait_impls.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn trait_impl_by_impl_node_id(&self, val: i64) -> Vec<&TraitImpl> {
        self.trait_impls.iter().filter(|r| r.impl_node_id == val).collect()
    }

    pub fn constant_by_node_id(&self, val: i64) -> Vec<&Constant> {
        self.constants.iter().filter(|r| r.node_id == val).collect()
    }

    pub fn constant_by_name(&self, val: &str) -> Vec<&Constant> {
        self.constants.iter().filter(|r| r.name == val).collect()
    }

    pub fn constant_by_type_ref(&self, val: &str) -> Vec<&Constant> {
        self.constants.iter().filter(|r| r.type_ref == val).collect()
    }

    pub fn constant_by_has_value(&self, val: bool) -> Vec<&Constant> {
        self.constants.iter().filter(|r| r.has_value == val).collect()
    }

    pub fn expr_by_id(&self, val: i64) -> Vec<&Expr> {
        self.exprs.iter().filter(|r| r.id == val).collect()
    }

    pub fn expr_by_parent_id(&self, val: i64) -> Vec<&Expr> {
        self.exprs.iter().filter(|r| r.parent_id == val).collect()
    }

    pub fn expr_by_kind(&self, val: ExprKind) -> Vec<&Expr> {
        self.exprs.iter().filter(|r| r.kind == val).collect()
    }

    pub fn expr_by_ordinal(&self, val: i64) -> Vec<&Expr> {
        self.exprs.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn expr_by_value(&self, val: &str) -> Vec<&Expr> {
        self.exprs.iter().filter(|r| r.value == val).collect()
    }

    pub fn match_arm_by_match_id(&self, val: i64) -> Vec<&MatchArm> {
        self.match_arms.iter().filter(|r| r.match_id == val).collect()
    }

    pub fn match_arm_by_ordinal(&self, val: i64) -> Vec<&MatchArm> {
        self.match_arms.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn match_arm_by_body_expr_id(&self, val: i64) -> Vec<&MatchArm> {
        self.match_arms.iter().filter(|r| r.body_expr_id == val).collect()
    }

    pub fn match_arm_by_kind(&self, val: ArmKind) -> Vec<&MatchArm> {
        self.match_arms.iter().filter(|r| r.kind == val).collect()
    }

    pub fn match_pattern_by_match_id(&self, val: i64) -> Vec<&MatchPattern> {
        self.match_patterns.iter().filter(|r| r.match_id == val).collect()
    }

    pub fn match_pattern_by_arm_ordinal(&self, val: i64) -> Vec<&MatchPattern> {
        self.match_patterns.iter().filter(|r| r.arm_ordinal == val).collect()
    }

    pub fn match_pattern_by_pat_ordinal(&self, val: i64) -> Vec<&MatchPattern> {
        self.match_patterns.iter().filter(|r| r.pat_ordinal == val).collect()
    }

    pub fn match_pattern_by_value(&self, val: &str) -> Vec<&MatchPattern> {
        self.match_patterns.iter().filter(|r| r.value == val).collect()
    }

    pub fn grammar_rule_by_node_id(&self, val: i64) -> Vec<&GrammarRule> {
        self.grammar_rules.iter().filter(|r| r.node_id == val).collect()
    }

    pub fn grammar_rule_by_rule_name(&self, val: &str) -> Vec<&GrammarRule> {
        self.grammar_rules.iter().filter(|r| r.rule_name == val).collect()
    }

    pub fn grammar_arm_by_rule_id(&self, val: i64) -> Vec<&GrammarArm> {
        self.grammar_arms.iter().filter(|r| r.rule_id == val).collect()
    }

    pub fn grammar_arm_by_ordinal(&self, val: i64) -> Vec<&GrammarArm> {
        self.grammar_arms.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn grammar_arm_by_result_expr_id(&self, val: i64) -> Vec<&GrammarArm> {
        self.grammar_arms.iter().filter(|r| r.result_expr_id == val).collect()
    }

    pub fn grammar_pat_elem_by_rule_id(&self, val: i64) -> Vec<&GrammarPatElem> {
        self.grammar_pat_elems.iter().filter(|r| r.rule_id == val).collect()
    }

    pub fn grammar_pat_elem_by_arm_ordinal(&self, val: i64) -> Vec<&GrammarPatElem> {
        self.grammar_pat_elems.iter().filter(|r| r.arm_ordinal == val).collect()
    }

    pub fn grammar_pat_elem_by_elem_ordinal(&self, val: i64) -> Vec<&GrammarPatElem> {
        self.grammar_pat_elems.iter().filter(|r| r.elem_ordinal == val).collect()
    }

    pub fn grammar_pat_elem_by_kind(&self, val: PatElemKind) -> Vec<&GrammarPatElem> {
        self.grammar_pat_elems.iter().filter(|r| r.kind == val).collect()
    }

    pub fn grammar_pat_elem_by_name(&self, val: &str) -> Vec<&GrammarPatElem> {
        self.grammar_pat_elems.iter().filter(|r| r.name == val).collect()
    }

    pub fn qualified_name_by_node_id(&self, val: i64) -> Vec<&QualifiedName> {
        self.qualified_names.iter().filter(|r| r.node_id == val).collect()
    }

    pub fn qualified_name_by_full_path(&self, val: &str) -> Vec<&QualifiedName> {
        self.qualified_names.iter().filter(|r| r.full_path == val).collect()
    }

    pub fn can_see_by_observer_id(&self, val: i64) -> Vec<&CanSee> {
        self.can_sees.iter().filter(|r| r.observer_id == val).collect()
    }

    pub fn can_see_by_visible_id(&self, val: i64) -> Vec<&CanSee> {
        self.can_sees.iter().filter(|r| r.visible_id == val).collect()
    }

    pub fn variant_of_by_variant_name(&self, val: &str) -> Vec<&VariantOf> {
        self.variant_ofs.iter().filter(|r| r.variant_name == val).collect()
    }

    pub fn variant_of_by_domain_name(&self, val: &str) -> Vec<&VariantOf> {
        self.variant_ofs.iter().filter(|r| r.domain_name == val).collect()
    }

    pub fn variant_of_by_domain_node_id(&self, val: i64) -> Vec<&VariantOf> {
        self.variant_ofs.iter().filter(|r| r.domain_node_id == val).collect()
    }

    pub fn binding_info_by_expr_id(&self, val: i64) -> Vec<&BindingInfo> {
        self.binding_infos.iter().filter(|r| r.expr_id == val).collect()
    }

    pub fn binding_info_by_var_name(&self, val: &str) -> Vec<&BindingInfo> {
        self.binding_infos.iter().filter(|r| r.var_name == val).collect()
    }

    pub fn binding_info_by_type_name(&self, val: &str) -> Vec<&BindingInfo> {
        self.binding_infos.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn type_kind_by_type_name(&self, val: &str) -> Vec<&TypeKind> {
        self.type_kinds.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn type_kind_by_category(&self, val: TypeCategory) -> Vec<&TypeKind> {
        self.type_kinds.iter().filter(|r| r.category == val).collect()
    }

    pub fn method_on_type_by_type_name(&self, val: &str) -> Vec<&MethodOnType> {
        self.method_on_types.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn method_on_type_by_method_name(&self, val: &str) -> Vec<&MethodOnType> {
        self.method_on_types.iter().filter(|r| r.method_name == val).collect()
    }

    pub fn method_on_type_by_method_node_id(&self, val: i64) -> Vec<&MethodOnType> {
        self.method_on_types.iter().filter(|r| r.method_node_id == val).collect()
    }

    pub fn contained_type_by_parent_type(&self, val: &str) -> Vec<&ContainedType> {
        self.contained_types.iter().filter(|r| r.parent_type == val).collect()
    }

    pub fn contained_type_by_child_type(&self, val: &str) -> Vec<&ContainedType> {
        self.contained_types.iter().filter(|r| r.child_type == val).collect()
    }

    pub fn recursive_type_by_parent_type(&self, val: &str) -> Vec<&RecursiveType> {
        self.recursive_types.iter().filter(|r| r.parent_type == val).collect()
    }

    pub fn recursive_type_by_child_type(&self, val: &str) -> Vec<&RecursiveType> {
        self.recursive_types.iter().filter(|r| r.child_type == val).collect()
    }

    pub fn operator_impl_by_trait_name(&self, val: &str) -> Vec<&OperatorImpl> {
        self.operator_impls.iter().filter(|r| r.trait_name == val).collect()
    }

    pub fn operator_impl_by_type_name(&self, val: &str) -> Vec<&OperatorImpl> {
        self.operator_impls.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn operator_impl_by_rust_import(&self, val: &str) -> Vec<&OperatorImpl> {
        self.operator_impls.iter().filter(|r| r.rust_import == val).collect()
    }

    pub fn operator_impl_by_output_type(&self, val: &str) -> Vec<&OperatorImpl> {
        self.operator_impls.iter().filter(|r| r.output_type == val).collect()
    }

    pub fn recursive_field_by_struct_name(&self, val: &str) -> Vec<&RecursiveField> {
        self.recursive_fields.iter().filter(|r| r.struct_name == val).collect()
    }

    pub fn recursive_field_by_field_name(&self, val: &str) -> Vec<&RecursiveField> {
        self.recursive_fields.iter().filter(|r| r.field_name == val).collect()
    }

    pub fn mutable_binding_by_expr_id(&self, val: i64) -> Vec<&MutableBinding> {
        self.mutable_bindings.iter().filter(|r| r.expr_id == val).collect()
    }

    pub fn mutable_binding_by_var_name(&self, val: &str) -> Vec<&MutableBinding> {
        self.mutable_bindings.iter().filter(|r| r.var_name == val).collect()
    }

    pub fn mutable_binding_by_type_name(&self, val: &str) -> Vec<&MutableBinding> {
        self.mutable_bindings.iter().filter(|r| r.type_name == val).collect()
    }

    /// Run all derivation rules to fixed point.
    pub fn derive(&mut self) {
        self.derive_variant_of();
        self.derive_type_kind();
        self.derive_binding_info();
        self.derive_method_on_type();
        self.derive_contained_type();
        self.derive_operator_impl();
        self.derive_mutable_binding();
        self.derive_qualified_names_fixpoint();
        self.derive_can_see_fixpoint();
        self.derive_recursive_type_fixpoint();
        self.derive_recursive_field();
    }

    fn derive_variant_of(&mut self) {
        let mut results = Vec::new();
        for node in &self.nodes {
            if node.kind == NodeKind::Domain {
                for variant in &self.variants {
                    if variant.domain_id == node.id {
                        results.push(VariantOf { variant_name: variant.name.clone(), domain_name: node.name.clone(), domain_node_id: node.id });
                    }
                }
            }
        }
        self.variant_ofs = results;
    }

    fn derive_type_kind(&mut self) {
        let mut results = Vec::new();
        for node in &self.nodes {
            if node.kind == NodeKind::Domain {
                results.push(TypeKind { type_name: node.name.clone(), category: TypeCategory::Domain });
            }
        }
        for node in &self.nodes {
            if node.kind == NodeKind::Struct {
                results.push(TypeKind { type_name: node.name.clone(), category: TypeCategory::Struct });
            }
        }
        self.type_kinds = results;
    }

    fn derive_binding_info(&mut self) {
        let mut results = Vec::new();
        for expr in &self.exprs {
            if expr.kind == ExprKind::SubTypeNew {
                if expr.value.contains(":") {
                    results.push(BindingInfo { expr_id: expr.id, var_name: expr.value[..expr.value.find(':').unwrap()].to_string().clone(), type_name: expr.value[expr.value.find(':').unwrap()+1..].to_string().clone() });
                }
            }
        }
        for expr in &self.exprs {
            if expr.kind == ExprKind::SameTypeNew {
                if !expr.value.contains(":") {
                    results.push(BindingInfo { expr_id: expr.id, var_name: expr.value.clone(), type_name: expr.value.clone() });
                }
            }
        }
        self.binding_infos = results;
    }

    fn derive_method_on_type(&mut self) {
        let mut results = Vec::new();
        for trait_impl in &self.trait_impls {
            for node in &self.nodes {
                if (node.kind == NodeKind::Method || node.kind == NodeKind::TailMethod) {
                    if node.parent == trait_impl.impl_node_id {
                        results.push(MethodOnType { type_name: trait_impl.type_name.clone(), method_name: node.name.clone(), method_node_id: node.id });
                    }
                }
            }
        }
        self.method_on_types = results;
    }

    fn derive_contained_type(&mut self) {
        let mut results = Vec::new();
        for node in &self.nodes {
            if node.kind == NodeKind::Struct {
                for field in &self.fields {
                    if field.struct_id == node.id {
                        results.push(ContainedType { parent_type: node.name.clone(), child_type: field.type_ref.clone() });
                    }
                }
            }
        }
        for node in &self.nodes {
            if node.kind == NodeKind::Domain {
                for variant in &self.variants {
                    if variant.domain_id == node.id {
                        if !variant.wraps_type.is_empty() {
                            results.push(ContainedType { parent_type: node.name.clone(), child_type: variant.wraps_type.clone() });
                        }
                    }
                }
            }
        }
        self.contained_types = results;
    }

    fn derive_operator_impl(&mut self) {
        let mut results = Vec::new();
        for trait_impl in &self.trait_impls {
            if (((((((trait_impl.trait_name == "add" || trait_impl.trait_name == "sub") || trait_impl.trait_name == "mul") || trait_impl.trait_name == "div") || trait_impl.trait_name == "rem") || trait_impl.trait_name == "neg") || trait_impl.trait_name == "not") || trait_impl.trait_name == "index") {
                results.push(OperatorImpl { trait_name: trait_impl.trait_name.clone(), type_name: trait_impl.type_name.clone(), rust_import: trait_impl.trait_name.clone(), output_type: trait_impl.type_name.clone() });
            }
        }
        self.operator_impls = results;
    }

    fn derive_recursive_field(&mut self) {
        let mut results = Vec::new();
        for node in &self.nodes {
            if node.kind == NodeKind::Struct {
                for field in &self.fields {
                    if field.struct_id == node.id {
                        for reach in &self.recursive_types {
                            if reach.parent_type == node.name {
                                if reach.child_type == node.name {
                                    results.push(RecursiveField { struct_name: node.name.clone(), field_name: field.name.clone() });
                                }
                            }
                        }
                    }
                }
            }
        }
        self.recursive_fields = results;
    }

    fn derive_mutable_binding(&mut self) {
        let mut results = Vec::new();
        for expr in &self.exprs {
            if expr.kind == ExprKind::MutableNew {
                if expr.value.contains(":") {
                    results.push(MutableBinding { expr_id: expr.id, var_name: expr.value[..expr.value.find(':').unwrap()].to_string().clone(), type_name: expr.value[expr.value.find(':').unwrap()+1..].to_string().clone() });
                }
            }
        }
        for expr in &self.exprs {
            if expr.kind == ExprKind::MutableNew {
                if !expr.value.contains(":") {
                    results.push(MutableBinding { expr_id: expr.id, var_name: expr.value.clone(), type_name: expr.value.clone() });
                }
            }
        }
        self.mutable_bindings = results;
    }

    fn derive_qualified_names_fixpoint(&mut self) {
        {
            let mut results = Vec::new();
            for node in &self.nodes {
                if node.parent == 0 {
                    if node.scope_id == 0 {
                        results.push(QualifiedName { node_id: node.id, full_path: node.name.clone() });
                    }
                }
            }
            for node in &self.nodes {
                if node.parent == 0 {
                    if node.scope_id != 0 {
                        for scope in &self.scopes {
                            if scope.id == node.scope_id {
                                results.push(QualifiedName { node_id: node.id, full_path: format!("{}::{}", scope.name, node.name).clone() });
                            }
                        }
                    }
                }
            }
            self.qualified_names = results;
        }
        loop {
            let mut new_items = Vec::new();
            for node in &self.nodes {
                if node.parent != 0 {
                    for qualified_name in &self.qualified_names {
                        if qualified_name.node_id == node.parent {
                            new_items.push(QualifiedName { node_id: node.id, full_path: format!("{}::{}", qualified_name.full_path, node.name).clone() });
                        }
                    }
                }
            }
            new_items.retain(|item| !self.qualified_names.contains(item));
            if new_items.is_empty() { break; }
            self.qualified_names.extend(new_items);
        }
    }

    fn derive_can_see_fixpoint(&mut self) {
        {
            let mut results = Vec::new();
            for node in &self.nodes {
                results.push(CanSee { observer_id: node.id, visible_id: node.id });
            }
            for node in &self.nodes {
                for other in &self.nodes {
                    if other.parent == node.parent {
                        if other.id != node.id {
                            results.push(CanSee { observer_id: node.id, visible_id: other.id });
                        }
                    }
                }
            }
            for node in &self.nodes {
                if node.scope_id != 0 {
                    for import in &self.imports {
                        if import.scope_id == node.scope_id {
                            for target in &self.nodes {
                                if target.name == import.imported_name {
                                    results.push(CanSee { observer_id: node.id, visible_id: target.id });
                                }
                            }
                        }
                    }
                }
            }
            self.can_sees = results;
        }
        loop {
            let mut new_items = Vec::new();
            for node in &self.nodes {
                if node.parent != 0 {
                    for edge in &self.can_sees {
                        if edge.observer_id == node.parent {
                            new_items.push(CanSee { observer_id: node.id, visible_id: edge.visible_id });
                        }
                    }
                }
            }
            new_items.retain(|item| !self.can_sees.contains(item));
            if new_items.is_empty() { break; }
            self.can_sees.extend(new_items);
        }
    }

    fn derive_recursive_type_fixpoint(&mut self) {
        {
            let mut results = Vec::new();
            for contained_type in &self.contained_types {
                results.push(RecursiveType { parent_type: contained_type.parent_type.clone(), child_type: contained_type.child_type.clone() });
            }
            self.recursive_types = results;
        }
        loop {
            let mut new_items = Vec::new();
            for contained_type in &self.contained_types {
                for reach in &self.recursive_types {
                    if reach.parent_type == contained_type.child_type {
                        new_items.push(RecursiveType { parent_type: contained_type.parent_type.clone(), child_type: reach.child_type.clone() });
                    }
                }
            }
            new_items.retain(|item| !self.recursive_types.contains(item));
            if new_items.is_empty() { break; }
            self.recursive_types.extend(new_items);
        }
    }

}
