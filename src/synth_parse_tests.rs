#[cfg(test)]
mod tests {
    use crate::synth_lex::SynthLexer;
    use crate::synth_parse::SynthParser;
    use synth_core::*;

    fn parse(source: &str, kind: DialectKind) -> Dialect {
        let tokens = SynthLexer::new(source).lex().unwrap();
        SynthParser::new(&tokens).parse(SurfaceKind::Aski, kind).unwrap()
    }

    #[test]
    fn sequential_rule() {
        let d = parse(":Constructor +<Type>", DialectKind::TypeApplication);
        assert_eq!(d.rules.len(), 1);
        match &d.rules[0] {
            Rule::Sequential { items } => {
                assert_eq!(items.len(), 2);
                match &items[0].content {
                    ItemContent::Named { label } => {
                        assert_eq!(label.binding, Binding::Reference);
                        assert_eq!(label.kind, LabelKind::Constructor);
                    }
                    _ => panic!("expected named"),
                }
            }
            _ => panic!("expected sequential"),
        }
    }

    #[test]
    fn ordered_choice() {
        let d = parse("// :Variant\n// (:Variant <Type>)", DialectKind::Enum);
        assert_eq!(d.rules.len(), 1);
        match &d.rules[0] {
            Rule::OrderedChoice { alternatives } => assert_eq!(alternatives.len(), 2),
            _ => panic!("expected ordered choice"),
        }
    }

    #[test]
    fn declare_vs_reference() {
        let d = parse("@EnumName :Type", DialectKind::Root);
        match &d.rules[0] {
            Rule::Sequential { items } => {
                match &items[0].content {
                    ItemContent::Named { label } => assert_eq!(label.binding, Binding::Declare),
                    _ => panic!("expected named"),
                }
                match &items[1].content {
                    ItemContent::Named { label } => assert_eq!(label.binding, Binding::Reference),
                    _ => panic!("expected named"),
                }
            }
            _ => panic!("expected sequential"),
        }
    }

    #[test]
    fn keyword_token() {
        let d = parse("Self", DialectKind::Param);
        match &d.rules[0] {
            Rule::Sequential { items } => {
                assert!(matches!(&items[0].content, ItemContent::Keyword { token: KeywordToken::Self_ }));
            }
            _ => panic!("expected sequential"),
        }
    }

    #[test]
    fn adjacency_preserved() {
        let d = parse("_@_:Instance", DialectKind::ExprAtom);
        match &d.rules[0] {
            Rule::Sequential { items } => {
                assert!(!items[0].adjacent);
                assert!(items[1].adjacent);
            }
            _ => panic!("expected sequential"),
        }
    }

    #[test]
    fn repeat_item() {
        let d = parse("+<Param>", DialectKind::Signature);
        match &d.rules[0] {
            Rule::Sequential { items } => {
                match &items[0].content {
                    ItemContent::Repeat { kind, inner } => {
                        assert_eq!(*kind, Cardinality::OneOrMore);
                        match &inner.content {
                            ItemContent::DialectRef { surface, target } => {
                                assert_eq!(*surface, None);
                                assert_eq!(*target, DialectKind::Param);
                            }
                            _ => panic!("expected dialect ref"),
                        }
                    }
                    _ => panic!("expected repeat"),
                }
            }
            _ => panic!("expected sequential"),
        }
    }

    #[test]
    fn parse_all_synth_files() {
        let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("source");
        if !source_root.exists() { return; }

        for surface_entry in std::fs::read_dir(&source_root).unwrap() {
            let surface_path = surface_entry.unwrap().path();
            if !surface_path.is_dir() { continue; }
            let surface_name = surface_path.file_name().unwrap().to_string_lossy().to_string();
            let surface_kind = SynthLexer::resolve_surface_kind(&surface_name)
                .unwrap_or_else(|e| panic!("unknown surface {}: {}", surface_name, e));
            for entry in std::fs::read_dir(&surface_path).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().map(|x| x == "synth").unwrap_or(false) {
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    let source = std::fs::read_to_string(&path).unwrap();
                    let tokens = SynthLexer::new(&source).lex()
                        .unwrap_or_else(|e| panic!("lex failed {}: {}", name, e));
                    let kind = SynthLexer::resolve_filename(&name)
                        .unwrap_or_else(|e| panic!("unknown dialect {}: {}", name, e));
                    let dialect = SynthParser::new(&tokens).parse(surface_kind, kind)
                        .unwrap_or_else(|e| panic!("parse failed {}: {}", name, e));
                    assert!(!dialect.rules.is_empty(), "no rules in {}", name);
                }
            }
        }
    }
}
