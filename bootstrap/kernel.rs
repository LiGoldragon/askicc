#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeForm {
    Domain,
    Struct,
}

impl std::fmt::Display for TypeForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TypeForm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "domain" => Some(Self::Domain),
            "struct" => Some(Self::Struct),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Struct => "struct",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatElemKind {
    Terminal,
    NonTerminal,
    Binding,
    BindType,
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
            "non_terminal" => Some(Self::NonTerminal),
            "NonTerminal" => Some(Self::NonTerminal),
            "binding" => Some(Self::Binding),
            "bind_type" => Some(Self::BindType),
            "BindType" => Some(Self::BindType),
            "bind_lit" => Some(Self::BindLit),
            "BindLit" => Some(Self::BindLit),
            "rest" => Some(Self::Rest),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::NonTerminal => "non_terminal",
            Self::Binding => "binding",
            Self::BindType => "bind_type",
            Self::BindLit => "bind_lit",
            Self::Rest => "rest",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultElemKind {
    Create,
    Bind,
    Recurse,
    Literal,
}

impl std::fmt::Display for ResultElemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResultElemKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "create" => Some(Self::Create),
            "bind" => Some(Self::Bind),
            "recurse" => Some(Self::Recurse),
            "literal" => Some(Self::Literal),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Bind => "bind",
            Self::Recurse => "recurse",
            Self::Literal => "literal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldValueKind {
    Ordinal,
    StringVal,
    Ref,
}

impl std::fmt::Display for FieldValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FieldValueKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ordinal" => Some(Self::Ordinal),
            "string_val" => Some(Self::StringVal),
            "StringVal" => Some(Self::StringVal),
            "ref" => Some(Self::Ref),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Ordinal => "ordinal",
            Self::StringVal => "string_val",
            Self::Ref => "ref",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustSpan {
    Cast,
    MethodCall,
    FreeCall,
    BlockExpr,
    IndexAccess,
}

impl std::fmt::Display for RustSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RustSpan {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cast" => Some(Self::Cast),
            "method_call" => Some(Self::MethodCall),
            "MethodCall" => Some(Self::MethodCall),
            "free_call" => Some(Self::FreeCall),
            "FreeCall" => Some(Self::FreeCall),
            "block_expr" => Some(Self::BlockExpr),
            "BlockExpr" => Some(Self::BlockExpr),
            "index_access" => Some(Self::IndexAccess),
            "IndexAccess" => Some(Self::IndexAccess),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Cast => "cast",
            Self::MethodCall => "method_call",
            Self::FreeCall => "free_call",
            Self::BlockExpr => "block_expr",
            Self::IndexAccess => "index_access",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub id: i64,
    pub name: String,
    pub form: TypeForm,
    pub parent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub type_id: i64,
    pub ordinal: i64,
    pub name: String,
    pub contains_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub type_id: i64,
    pub ordinal: i64,
    pub name: String,
    pub field_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: i64,
    pub name: String,
    pub dialect: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Arm {
    pub rule_id: i64,
    pub ordinal: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatElem {
    pub rule_id: i64,
    pub arm_ordinal: i64,
    pub elem_ordinal: i64,
    pub kind: PatElemKind,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultElem {
    pub rule_id: i64,
    pub arm_ordinal: i64,
    pub elem_ordinal: i64,
    pub kind: ResultElemKind,
    pub type_name: String,
    pub field_name: String,
    pub binding_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FfiEntry {
    pub library: String,
    pub aski_name: String,
    pub rust_name: String,
    pub span: RustSpan,
    pub return_type: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Instance {
    pub id: i64,
    pub type_id: i64,
    pub parent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldValue {
    pub instance_id: i64,
    pub field_ordinal: i64,
    pub value_kind: FieldValueKind,
    pub ordinal_value: i64,
    pub string_value: String,
    pub ref_value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantOf {
    pub variant_name: String,
    pub type_name: String,
    pub type_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeKind {
    pub type_name: String,
    pub category: TypeForm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainedType {
    pub parent_type: String,
    pub child_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveType {
    pub parent_type: String,
    pub child_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub types: Vec<Type>,
    pub variants: Vec<Variant>,
    pub fields: Vec<Field>,
    pub rules: Vec<Rule>,
    pub arms: Vec<Arm>,
    pub pat_elems: Vec<PatElem>,
    pub result_elems: Vec<ResultElem>,
    pub ffi_entries: Vec<FfiEntry>,
    pub instances: Vec<Instance>,
    pub field_values: Vec<FieldValue>,
    pub variant_ofs: Vec<VariantOf>,
    pub type_kinds: Vec<TypeKind>,
    pub contained_types: Vec<ContainedType>,
    pub recursive_types: Vec<RecursiveType>,
}

impl Default for World { fn default() -> Self { Self { types: Default::default(), variants: Default::default(), fields: Default::default(), rules: Default::default(), arms: Default::default(), pat_elems: Default::default(), result_elems: Default::default(), ffi_entries: Default::default(), instances: Default::default(), field_values: Default::default(), variant_ofs: Default::default(), type_kinds: Default::default(), contained_types: Default::default(), recursive_types: Default::default(), } } }

impl World {
    pub fn new() -> Self { Self::default() }

    pub fn type_by_id(&self, val: i64) -> Vec<&Type> {
        self.types.iter().filter(|r| r.id == val).collect()
    }

    pub fn type_by_name(&self, val: &str) -> Vec<&Type> {
        self.types.iter().filter(|r| r.name == val).collect()
    }

    pub fn type_by_form(&self, val: TypeForm) -> Vec<&Type> {
        self.types.iter().filter(|r| r.form == val).collect()
    }

    pub fn type_by_parent(&self, val: i64) -> Vec<&Type> {
        self.types.iter().filter(|r| r.parent == val).collect()
    }

    pub fn variant_by_type_id(&self, val: i64) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.type_id == val).collect()
    }

    pub fn variant_by_ordinal(&self, val: i64) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn variant_by_name(&self, val: &str) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.name == val).collect()
    }

    pub fn variant_by_contains_type(&self, val: &str) -> Vec<&Variant> {
        self.variants.iter().filter(|r| r.contains_type == val).collect()
    }

    pub fn field_by_type_id(&self, val: i64) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.type_id == val).collect()
    }

    pub fn field_by_ordinal(&self, val: i64) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn field_by_name(&self, val: &str) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.name == val).collect()
    }

    pub fn field_by_field_type(&self, val: &str) -> Vec<&Field> {
        self.fields.iter().filter(|r| r.field_type == val).collect()
    }

    pub fn rule_by_id(&self, val: i64) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.id == val).collect()
    }

    pub fn rule_by_name(&self, val: &str) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.name == val).collect()
    }

    pub fn rule_by_dialect(&self, val: &str) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.dialect == val).collect()
    }

    pub fn arm_by_rule_id(&self, val: i64) -> Vec<&Arm> {
        self.arms.iter().filter(|r| r.rule_id == val).collect()
    }

    pub fn arm_by_ordinal(&self, val: i64) -> Vec<&Arm> {
        self.arms.iter().filter(|r| r.ordinal == val).collect()
    }

    pub fn pat_elem_by_rule_id(&self, val: i64) -> Vec<&PatElem> {
        self.pat_elems.iter().filter(|r| r.rule_id == val).collect()
    }

    pub fn pat_elem_by_arm_ordinal(&self, val: i64) -> Vec<&PatElem> {
        self.pat_elems.iter().filter(|r| r.arm_ordinal == val).collect()
    }

    pub fn pat_elem_by_elem_ordinal(&self, val: i64) -> Vec<&PatElem> {
        self.pat_elems.iter().filter(|r| r.elem_ordinal == val).collect()
    }

    pub fn pat_elem_by_kind(&self, val: PatElemKind) -> Vec<&PatElem> {
        self.pat_elems.iter().filter(|r| r.kind == val).collect()
    }

    pub fn pat_elem_by_name(&self, val: &str) -> Vec<&PatElem> {
        self.pat_elems.iter().filter(|r| r.name == val).collect()
    }

    pub fn result_elem_by_rule_id(&self, val: i64) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.rule_id == val).collect()
    }

    pub fn result_elem_by_arm_ordinal(&self, val: i64) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.arm_ordinal == val).collect()
    }

    pub fn result_elem_by_elem_ordinal(&self, val: i64) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.elem_ordinal == val).collect()
    }

    pub fn result_elem_by_kind(&self, val: ResultElemKind) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.kind == val).collect()
    }

    pub fn result_elem_by_type_name(&self, val: &str) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn result_elem_by_field_name(&self, val: &str) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.field_name == val).collect()
    }

    pub fn result_elem_by_binding_name(&self, val: &str) -> Vec<&ResultElem> {
        self.result_elems.iter().filter(|r| r.binding_name == val).collect()
    }

    pub fn ffi_entry_by_library(&self, val: &str) -> Vec<&FfiEntry> {
        self.ffi_entries.iter().filter(|r| r.library == val).collect()
    }

    pub fn ffi_entry_by_aski_name(&self, val: &str) -> Vec<&FfiEntry> {
        self.ffi_entries.iter().filter(|r| r.aski_name == val).collect()
    }

    pub fn ffi_entry_by_rust_name(&self, val: &str) -> Vec<&FfiEntry> {
        self.ffi_entries.iter().filter(|r| r.rust_name == val).collect()
    }

    pub fn ffi_entry_by_span(&self, val: RustSpan) -> Vec<&FfiEntry> {
        self.ffi_entries.iter().filter(|r| r.span == val).collect()
    }

    pub fn ffi_entry_by_return_type(&self, val: &str) -> Vec<&FfiEntry> {
        self.ffi_entries.iter().filter(|r| r.return_type == val).collect()
    }

    pub fn instance_by_id(&self, val: i64) -> Vec<&Instance> {
        self.instances.iter().filter(|r| r.id == val).collect()
    }

    pub fn instance_by_type_id(&self, val: i64) -> Vec<&Instance> {
        self.instances.iter().filter(|r| r.type_id == val).collect()
    }

    pub fn instance_by_parent(&self, val: i64) -> Vec<&Instance> {
        self.instances.iter().filter(|r| r.parent == val).collect()
    }

    pub fn field_value_by_instance_id(&self, val: i64) -> Vec<&FieldValue> {
        self.field_values.iter().filter(|r| r.instance_id == val).collect()
    }

    pub fn field_value_by_field_ordinal(&self, val: i64) -> Vec<&FieldValue> {
        self.field_values.iter().filter(|r| r.field_ordinal == val).collect()
    }

    pub fn field_value_by_value_kind(&self, val: FieldValueKind) -> Vec<&FieldValue> {
        self.field_values.iter().filter(|r| r.value_kind == val).collect()
    }

    pub fn field_value_by_ordinal_value(&self, val: i64) -> Vec<&FieldValue> {
        self.field_values.iter().filter(|r| r.ordinal_value == val).collect()
    }

    pub fn field_value_by_string_value(&self, val: &str) -> Vec<&FieldValue> {
        self.field_values.iter().filter(|r| r.string_value == val).collect()
    }

    pub fn field_value_by_ref_value(&self, val: i64) -> Vec<&FieldValue> {
        self.field_values.iter().filter(|r| r.ref_value == val).collect()
    }

    pub fn variant_of_by_variant_name(&self, val: &str) -> Vec<&VariantOf> {
        self.variant_ofs.iter().filter(|r| r.variant_name == val).collect()
    }

    pub fn variant_of_by_type_name(&self, val: &str) -> Vec<&VariantOf> {
        self.variant_ofs.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn variant_of_by_type_id(&self, val: i64) -> Vec<&VariantOf> {
        self.variant_ofs.iter().filter(|r| r.type_id == val).collect()
    }

    pub fn type_kind_by_type_name(&self, val: &str) -> Vec<&TypeKind> {
        self.type_kinds.iter().filter(|r| r.type_name == val).collect()
    }

    pub fn type_kind_by_category(&self, val: TypeForm) -> Vec<&TypeKind> {
        self.type_kinds.iter().filter(|r| r.category == val).collect()
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

}

pub trait Derive {
    fn derive(&mut self);
    fn derive_variant_of(&mut self);
    fn derive_type_kind(&mut self);
    fn derive_contained_type(&mut self);
    fn derive_recursive_type(&mut self);
}

impl World {
    pub fn derive(&mut self) {
        self.derive_variant_of();
        self.derive_type_kind();
        self.derive_contained_type();
        self.derive_recursive_type_fixpoint();
    }

    fn derive_variant_of(&mut self) {
        let mut results = Vec::new();
        for type_entry in &self.types {
            if type_entry.form == TypeForm::Domain {
                for variant in &self.variants {
                    if variant.type_id == type_entry.id {
                        results.push(VariantOf { variant_name: variant.name.clone(), type_name: type_entry.name.clone(), type_id: type_entry.id });
                    }
                }
            }
        }
        self.variant_ofs = results;
    }

    fn derive_type_kind(&mut self) {
        let mut results = Vec::new();
        for type_entry in &self.types {
            results.push(TypeKind { type_name: type_entry.name.clone(), category: type_entry.form });
        }
        self.type_kinds = results;
    }

    fn derive_contained_type(&mut self) {
        let mut results = Vec::new();
        for type_entry in &self.types {
            if type_entry.form == TypeForm::Struct {
                for field in &self.fields {
                    if field.type_id == type_entry.id {
                        results.push(ContainedType { parent_type: type_entry.name.clone(), child_type: field.field_type.clone() });
                    }
                }
            }
        }
        for type_entry in &self.types {
            if type_entry.form == TypeForm::Domain {
                for variant in &self.variants {
                    if variant.type_id == type_entry.id {
                        if !variant.contains_type.is_empty() {
                            results.push(ContainedType { parent_type: type_entry.name.clone(), child_type: variant.contains_type.clone() });
                        }
                    }
                }
            }
        }
        self.contained_types = results;
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
