#[cfg(test)]
mod tests {
    use crate::synth_lex::SynthLexer;
    use crate::synth_token::SynthToken;
    use synth_core::*;

    #[test]
    fn lex_declare() {
        let tokens = SynthLexer::new("@EnumName").lex().unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Declare);
                assert_eq!(l.kind, LabelKind::EnumName);
                assert_eq!(l.casing, Casing::Pascal);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_reference() {
        let tokens = SynthLexer::new(":Type").lex().unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Reference);
                assert_eq!(l.kind, LabelKind::Type_);
                assert_eq!(l.casing, Casing::Pascal);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_camel_declare() {
        let tokens = SynthLexer::new("@traitName").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Declare);
                assert_eq!(l.kind, LabelKind::TraitName);
                assert_eq!(l.casing, Casing::Camel);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_camel_reference() {
        let tokens = SynthLexer::new(":method").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Reference);
                assert_eq!(l.kind, LabelKind::Method);
                assert_eq!(l.casing, Casing::Camel);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_origin() {
        let tokens = SynthLexer::new("'PlaceName").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Origin);
                assert_eq!(l.kind, LabelKind::PlaceName);
                assert_eq!(l.casing, Casing::Pascal);
            }
            _ => panic!("expected origin label"),
        }
    }

    #[test]
    fn lex_tag() {
        let tokens = SynthLexer::new("#Enum#").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Tag(k) => assert_eq!(*k, TagKind::Enum),
            _ => panic!("expected tag"),
        }
    }

    #[test]
    fn lex_keyword() {
        let tokens = SynthLexer::new("self").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Keyword(KeywordToken::Self_)));
    }

    #[test]
    fn lex_dialect_ref() {
        let tokens = SynthLexer::new("<Type>").lex().unwrap();
        match &tokens[0].token {
            SynthToken::DialectRef { surface, target } => {
                assert_eq!(*surface, None);
                assert_eq!(*target, DialectKind::Type_);
            }
            _ => panic!("expected dialect ref"),
        }
    }

    #[test]
    fn lex_cross_surface_dialect_ref() {
        let tokens = SynthLexer::new("<:aski:Statement>").lex().unwrap();
        match &tokens[0].token {
            SynthToken::DialectRef { surface, target } => {
                assert_eq!(*surface, Some(SurfaceKind::Aski));
                assert_eq!(*target, DialectKind::Statement);
            }
            _ => panic!("expected cross-surface ref"),
        }
    }

    #[test]
    fn lex_literal_escape() {
        let tokens = SynthLexer::new("_@_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::At)));
    }

    #[test]
    fn lex_tilde_escape() {
        let tokens = SynthLexer::new("_~_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Tilde)));
    }

    #[test]
    fn lex_apostrophe_escape() {
        let tokens = SynthLexer::new("_'_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Apostrophe)));
    }

    // ── v0.19 specific tests ──

    #[test]
    fn lex_ampersand_escape_v019() {
        let tokens = SynthLexer::new("_&_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Ampersand)));
    }

    #[test]
    fn lex_colon_escape_v019() {
        let tokens = SynthLexer::new("_:_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Colon)));
    }

    #[test]
    fn lex_local_construct_tag() {
        let tokens = SynthLexer::new("#LocalConstruct#").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Tag(k) => assert_eq!(*k, TagKind::LocalConstruct),
            _ => panic!("expected tag"),
        }
    }

    #[test]
    fn lex_local_canonical_tag() {
        let tokens = SynthLexer::new("#LocalCanonical#").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Tag(TagKind::LocalCanonical)));
    }

    #[test]
    fn lex_variant_alt_tag() {
        let tokens = SynthLexer::new("#VariantAlt#").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Tag(TagKind::VariantAlt)));
    }

    #[test]
    fn lex_exported_name_label() {
        let tokens = SynthLexer::new("@ExportedName").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Declare);
                assert_eq!(l.kind, LabelKind::ExportedName);
                assert_eq!(l.casing, Casing::Pascal);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_imported_name_reference() {
        let tokens = SynthLexer::new(":ImportedName").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Reference);
                assert_eq!(l.kind, LabelKind::ImportedName);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_instance_name_camel() {
        let tokens = SynthLexer::new("@instanceName").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Declare);
                assert_eq!(l.kind, LabelKind::InstanceName);
                assert_eq!(l.casing, Casing::Camel);
            }
            _ => panic!("expected label"),
        }
    }

    #[test]
    fn lex_borrow_param_v019() {
        // _&_self — shared borrow of self
        let tokens = SynthLexer::new("_&_self").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Ampersand)));
        assert!(matches!(&tokens[1].token, SynthToken::Keyword(KeywordToken::Self_)));
    }

    #[test]
    fn lex_mut_borrow_param_v019() {
        // _~__&_self — mutable borrow
        let tokens = SynthLexer::new("_~__&_self").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Tilde)));
        assert!(matches!(&tokens[1].token, SynthToken::Literal(LiteralToken::Ampersand)));
        assert!(matches!(&tokens[2].token, SynthToken::Keyword(KeywordToken::Self_)));
    }

    #[test]
    fn lex_type_app_brace_v019() {
        // Type application with {} (v0.19; was [])
        let tokens = SynthLexer::new("{ <TypeApplication> }").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Open(DelimKind::Brace)));
        assert!(matches!(&tokens[2].token, SynthToken::Close(DelimKind::Brace)));
    }

    #[test]
    fn lex_adjacency() {
        let tokens = SynthLexer::new("_@_:Instance").lex().unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(!tokens[0].adjacent);
        assert!(tokens[1].adjacent);
    }

    #[test]
    fn lex_spaced() {
        let tokens = SynthLexer::new("@EnumName <Type>").lex().unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(!tokens[1].adjacent);
    }

    #[test]
    fn lex_ordered_choice() {
        let tokens = SynthLexer::new("// @Variant\n// (@Variant <Type>)").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Or));
        assert!(matches!(&tokens[2].token, SynthToken::Or));
    }

    #[test]
    fn lex_cardinality() {
        let tokens = SynthLexer::new("*@Variant +<Param> ?<Type>").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::ZeroOrMore));
        assert!(matches!(&tokens[2].token, SynthToken::OneOrMore));
        assert!(matches!(&tokens[4].token, SynthToken::Optional));
    }

    #[test]
    fn lex_delimiters() {
        let tokens = SynthLexer::new("(@Variant <Type>)").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Open(DelimKind::Paren)));
        assert!(matches!(&tokens[3].token, SynthToken::Close(DelimKind::Paren)));
    }

    #[test]
    fn lex_piped_delimiters() {
        let tokens = SynthLexer::new("(|<Match>|)").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Open(DelimKind::ParenPipe)));
        assert!(matches!(&tokens[2].token, SynthToken::Close(DelimKind::ParenPipe)));
    }

    #[test]
    fn lex_bare_operators() {
        let tokens = SynthLexer::new("== != < > <= >=").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Eq)));
        assert!(matches!(&tokens[1].token, SynthToken::Literal(LiteralToken::NotEq)));
        assert!(matches!(&tokens[2].token, SynthToken::Literal(LiteralToken::Lt)));
        assert!(matches!(&tokens[3].token, SynthToken::Literal(LiteralToken::Gt)));
        assert!(matches!(&tokens[4].token, SynthToken::Literal(LiteralToken::LtEq)));
        assert!(matches!(&tokens[5].token, SynthToken::Literal(LiteralToken::GtEq)));
    }

    #[test]
    fn lex_comment() {
        let tokens = SynthLexer::new(";; comment\n@EnumName").lex().unwrap();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn lex_string_lit() {
        let tokens = SynthLexer::new("\"literal\"").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::StringLit));
    }

    #[test]
    fn lex_colon_as_operator() {
        // : not followed by alphabetic is a bare operator
        let tokens = SynthLexer::new(": ").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::Colon)));
    }

    #[test]
    fn lex_all_synth_files() {
        let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("source");
        if !source_root.exists() { return; }

        for surface_entry in std::fs::read_dir(&source_root).unwrap() {
            let surface_path = surface_entry.unwrap().path();
            if !surface_path.is_dir() { continue; }
            for entry in std::fs::read_dir(&surface_path).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().map(|x| x == "synth").unwrap_or(false) {
                    let source = std::fs::read_to_string(&path).unwrap();
                    let result = SynthLexer::new(&source).lex();
                    assert!(result.is_ok(), "failed to lex {}: {:?}", path.display(), result.err());
                }
            }
        }
    }
}
