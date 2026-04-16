/// Codegen — parsed .aski domains → Rust source code.
///
/// Generates the enum-as-index architecture:
/// - Enums → Rust enums
/// - Structs → Rust structs
/// - Newtypes → Rust tuple structs
/// - Type applications → Rust generic syntax

use crate::aski_parse::*;

pub struct RustOutput {
    pub source: String,
}

impl RustOutput {
    pub fn from_module(module: &Module) -> Self {
        let mut out = String::new();

        for domain in &module.domains {
            domain.emit(&mut out);
            out.push('\n');
        }

        RustOutput { source: out }
    }
}

trait Emit {
    fn emit(&self, out: &mut String);
}

impl Emit for Domain {
    fn emit(&self, out: &mut String) {
        match self {
            Domain::Enum(e) => e.emit(out),
            Domain::Struct(s) => s.emit(out),
            Domain::Newtype(n) => n.emit(out),
        }
    }
}

impl Emit for EnumDef {
    fn emit(&self, out: &mut String) {
        let has_data = self.variants.iter().any(|v| !matches!(v, EnumVariant::Bare(_)));

        if has_data {
            out.push_str("#[derive(Debug, Clone)]\n");
        } else {
            out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
        }
        out.push_str(&format!("pub enum {} {{\n", self.name));

        for variant in &self.variants {
            variant.emit(out);
        }

        out.push_str("}\n");
    }
}

impl Emit for EnumVariant {
    fn emit(&self, out: &mut String) {
        match self {
            EnumVariant::Bare(name) => {
                out.push_str(&format!("    {},\n", name));
            }
            EnumVariant::Data { name, payload } => {
                out.push_str(&format!("    {}({}),\n", name, payload.to_rust()));
            }
            EnumVariant::Struct(s) => {
                out.push_str(&format!("    {} {{\n", s.name));
                for field in &s.fields {
                    match field {
                        StructField::Typed { name, typ } => {
                            out.push_str(&format!("        {}: {},\n", to_snake(name), typ.to_rust()));
                        }
                        StructField::SelfTyped(name) => {
                            out.push_str(&format!("        {}: {},\n", to_snake(name), name));
                        }
                    }
                }
                out.push_str("    },\n");
            }
        }
    }
}

impl Emit for StructDef {
    fn emit(&self, out: &mut String) {
        out.push_str("#[derive(Debug, Clone)]\n");
        out.push_str(&format!("pub struct {} {{\n", self.name));

        for field in &self.fields {
            match field {
                StructField::Typed { name, typ } => {
                    out.push_str(&format!("    pub {}: {},\n", to_snake(name), typ.to_rust()));
                }
                StructField::SelfTyped(name) => {
                    out.push_str(&format!("    pub {}: {},\n", to_snake(name), name));
                }
            }
        }

        out.push_str("}\n");
    }
}

impl Emit for NewtypeDef {
    fn emit(&self, out: &mut String) {
        out.push_str("#[derive(Debug, Clone)]\n");
        out.push_str(&format!("pub struct {}({});\n", self.name, self.wraps.to_rust()));
    }
}

impl TypeExpr {
    fn to_rust(&self) -> String {
        match self {
            TypeExpr::Simple(name) => map_primitive(name).to_string(),
            TypeExpr::Application { constructor, args } => {
                let args_rust: Vec<String> = args.iter().map(|a| a.to_rust()).collect();
                format!("{}<{}>", constructor, args_rust.join(", "))
            }
        }
    }
}

fn map_primitive(name: &str) -> &str {
    match name {
        "U8" => "u8", "U16" => "u16", "U32" => "u32", "U64" => "u64",
        "I8" => "i8", "I16" => "i16", "I32" => "i32", "I64" => "i64",
        "F32" => "f32", "F64" => "f64",
        "Bool" => "bool", "String" => "String",
        other => other,
    }
}

fn to_snake(pascal: &str) -> String {
    let mut result = String::new();
    for (i, ch) in pascal.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_lowercase().next().unwrap());
    }
    // handle Rust reserved words
    match result.as_str() {
        "type" => "typ".to_string(),
        "self" => "self_".to_string(),
        "match" => "match_".to_string(),
        "loop" => "loop_".to_string(),
        "return" => "return_".to_string(),
        _ => result,
    }
}
