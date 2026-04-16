#[cfg(test)]
mod tests {
    use crate::aski_parse::*;

    fn parse(source: &str) -> Module {
        Module::parse(source).unwrap()
    }

    #[test]
    fn parse_bare_enum() {
        let m = parse("(Name NameDomain)\n(NameDomain Type Variant Field)");
        assert_eq!(m.name, "Name");
        assert_eq!(m.domains.len(), 1);
        match &m.domains[0] {
            Domain::Enum(e) => {
                assert_eq!(e.name, "NameDomain");
                assert_eq!(e.variants.len(), 3);
                match &e.variants[0] {
                    EnumVariant::Bare(name) => assert_eq!(name, "Type"),
                    other => panic!("expected Bare, got {:?}", other),
                }
            }
            other => panic!("expected Enum, got {:?}", other),
        }
    }

    #[test]
    fn parse_data_carrying_variant() {
        let m = parse("(Test Expr)\n(Expr (Int I64) (Float F64) None)");
        match &m.domains[0] {
            Domain::Enum(e) => {
                assert_eq!(e.variants.len(), 3);
                match &e.variants[0] {
                    EnumVariant::Data { name, payload } => {
                        assert_eq!(name, "Int");
                        match payload {
                            TypeExpr::Simple(t) => assert_eq!(t, "I64"),
                            other => panic!("expected Simple, got {:?}", other),
                        }
                    }
                    other => panic!("expected Data, got {:?}", other),
                }
                match &e.variants[2] {
                    EnumVariant::Bare(name) => assert_eq!(name, "None"),
                    other => panic!("expected Bare, got {:?}", other),
                }
            }
            other => panic!("expected Enum, got {:?}", other),
        }
    }

    #[test]
    fn parse_struct_variant_in_enum() {
        let m = parse("(Test Expr)\n(Expr {BinAdd (Left [Box Expr]) (Right [Box Expr])})");
        match &m.domains[0] {
            Domain::Enum(e) => {
                match &e.variants[0] {
                    EnumVariant::Struct(s) => {
                        assert_eq!(s.name, "BinAdd");
                        assert_eq!(s.fields.len(), 2);
                        match &s.fields[0] {
                            StructField::Typed { name, typ } => {
                                assert_eq!(name, "Left");
                                match typ {
                                    TypeExpr::Application { constructor, args } => {
                                        assert_eq!(constructor, "Box");
                                        assert_eq!(args.len(), 1);
                                    }
                                    other => panic!("expected Application, got {:?}", other),
                                }
                            }
                            other => panic!("expected Typed, got {:?}", other),
                        }
                    }
                    other => panic!("expected Struct variant, got {:?}", other),
                }
            }
            other => panic!("expected Enum, got {:?}", other),
        }
    }

    #[test]
    fn parse_struct_with_typed_and_self_typed() {
        let m = parse("(Test Block)\n{Block (Statements [Vec Statement]) (Tail [Option [Box Expr]])}");
        match &m.domains[0] {
            Domain::Struct(s) => {
                assert_eq!(s.name, "Block");
                assert_eq!(s.fields.len(), 2);
                match &s.fields[1] {
                    StructField::Typed { name, typ } => {
                        assert_eq!(name, "Tail");
                        match typ {
                            TypeExpr::Application { constructor, args } => {
                                assert_eq!(constructor, "Option");
                                assert_eq!(args.len(), 1);
                                match &args[0] {
                                    TypeExpr::Application { constructor, .. } => {
                                        assert_eq!(constructor, "Box");
                                    }
                                    other => panic!("expected nested App, got {:?}", other),
                                }
                            }
                            other => panic!("expected Application, got {:?}", other),
                        }
                    }
                    other => panic!("expected Typed, got {:?}", other),
                }
            }
            other => panic!("expected Struct, got {:?}", other),
        }
    }

    #[test]
    fn parse_self_typed_field() {
        let m = parse("(Test MyStruct)\n{MyStruct (Name TypeExpr) Span}");
        match &m.domains[0] {
            Domain::Struct(s) => {
                assert_eq!(s.fields.len(), 2);
                match &s.fields[1] {
                    StructField::SelfTyped(name) => assert_eq!(name, "Span"),
                    other => panic!("expected SelfTyped, got {:?}", other),
                }
            }
            other => panic!("expected Struct, got {:?}", other),
        }
    }

    #[test]
    fn parse_struct_variant_with_self_typed() {
        let m = parse("(Test Expr)\n(Expr {IntLit (Value I64) Span})");
        match &m.domains[0] {
            Domain::Enum(e) => {
                match &e.variants[0] {
                    EnumVariant::Struct(s) => {
                        assert_eq!(s.name, "IntLit");
                        assert_eq!(s.fields.len(), 2);
                        match &s.fields[1] {
                            StructField::SelfTyped(name) => assert_eq!(name, "Span"),
                            other => panic!("expected SelfTyped, got {:?}", other),
                        }
                    }
                    other => panic!("expected Struct variant, got {:?}", other),
                }
            }
            other => panic!("expected Enum, got {:?}", other),
        }
    }

    #[test]
    fn parse_full_root_aski() {
        let source = std::fs::read_to_string("aski/root.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Root");
        assert!(m.domains.len() > 5);
    }

    #[test]
    fn parse_full_expr_aski() {
        let source = std::fs::read_to_string("aski/expr.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Expr");
        assert!(m.domains.len() >= 2); // Expr enum + FieldInit struct
    }

    #[test]
    fn parse_full_body_aski() {
        let source = std::fs::read_to_string("aski/body.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Body");
    }

    #[test]
    fn parse_full_statement_aski() {
        let source = std::fs::read_to_string("aski/statement.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Statement");
    }

    #[test]
    fn parse_full_type_aski() {
        let source = std::fs::read_to_string("aski/type.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Type");
    }

    #[test]
    fn parse_full_trait_aski() {
        let source = std::fs::read_to_string("aski/trait.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Trait");
    }

    #[test]
    fn parse_full_pattern_aski() {
        let source = std::fs::read_to_string("aski/pattern.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Pattern");
    }

    #[test]
    fn parse_full_dialect_aski() {
        let source = std::fs::read_to_string("aski/dialect.aski").unwrap();
        let m = Module::parse(&source).unwrap();
        assert_eq!(m.name, "Dialect");
    }
}
