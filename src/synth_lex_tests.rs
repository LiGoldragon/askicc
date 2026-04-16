#[cfg(test)]
mod tests {
    use crate::synth_lex::SynthLexer;
    use crate::synth_token::SynthToken;
    use aski_core::*;

    #[test]
    fn lex_declare() {
        let tokens = SynthLexer::new("@Enum").lex().unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Declare);
                assert_eq!(l.kind, LabelKind::Enum);
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
        let tokens = SynthLexer::new("@trait").lex().unwrap();
        match &tokens[0].token {
            SynthToken::Label(l) => {
                assert_eq!(l.binding, Binding::Declare);
                assert_eq!(l.kind, LabelKind::Trait);
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
    fn lex_keyword() {
        let tokens = SynthLexer::new("Self").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Keyword(KeywordToken::Self_)));
    }

    #[test]
    fn lex_dialect_ref() {
        let tokens = SynthLexer::new("<Type>").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::DialectRef(DialectKind::Type_)));
    }

    #[test]
    fn lex_literal_escape() {
        let tokens = SynthLexer::new("_@_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::At)));
    }

    #[test]
    fn lex_mut_at_escape() {
        let tokens = SynthLexer::new("_~@_").lex().unwrap();
        assert!(matches!(&tokens[0].token, SynthToken::Literal(LiteralToken::MutAt)));
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
        let tokens = SynthLexer::new("@Enum <Type>").lex().unwrap();
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
        let tokens = SynthLexer::new(";; comment\n@Enum").lex().unwrap();
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
        let synth_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("source");
        if !synth_dir.exists() { return; }

        for entry in std::fs::read_dir(&synth_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().map(|x| x == "synth").unwrap_or(false) {
                let source = std::fs::read_to_string(&path).unwrap();
                let result = SynthLexer::new(&source).lex();
                assert!(result.is_ok(), "failed to lex {}: {:?}", path.display(), result.err());
            }
        }
    }
}
