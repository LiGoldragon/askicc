#[cfg(test)]
mod tests {
    use crate::synth_lex;
    use crate::synth_parse::*;

    fn parse_dialect(name: &str, source: &str) -> Dialect {
        let tokens = synth_lex::synth_lex(source).unwrap();
        Dialect::parse(name, &tokens).unwrap()
    }

    #[test]
    fn parse_sequential_rule() {
        let d = parse_dialect("Test", "(@Module <Module>)");
        assert_eq!(d.rules.len(), 1);
        match &d.rules[0] {
            Rule::Sequential(items) => {
                assert_eq!(items.len(), 1); // one delimited group
                match &items[0].content {
                    ItemContent::Delimited { kind, inner } => {
                        assert_eq!(*kind, DelimKind::Paren);
                        assert_eq!(inner.len(), 2); // @Module + <Module>
                    }
                    other => panic!("expected Delimited, got {:?}", other),
                }
            }
            other => panic!("expected Sequential, got {:?}", other),
        }
    }

    #[test]
    fn parse_ordered_choice() {
        let d = parse_dialect("Test", "// *@Variant\n// *(@Variant <Type>)");
        assert_eq!(d.rules.len(), 1);
        match &d.rules[0] {
            Rule::OrderedChoice(alts) => {
                assert_eq!(alts.len(), 2);
                assert_eq!(alts[0].cardinality, Cardinality::ZeroOrMore);
                assert_eq!(alts[1].cardinality, Cardinality::ZeroOrMore);
            }
            other => panic!("expected OrderedChoice, got {:?}", other),
        }
    }

    #[test]
    fn parse_literal_escape() {
        let d = parse_dialect("Test", "_@_@Name");
        match &d.rules[0] {
            Rule::Sequential(items) => {
                assert_eq!(items.len(), 2);
                match &items[0].content {
                    ItemContent::Literal(s) => assert_eq!(s, "@"),
                    other => panic!("expected Literal, got {:?}", other),
                }
                match &items[1].content {
                    ItemContent::Declare { label, .. } => assert_eq!(label, "Name"),
                    other => panic!("expected Declare, got {:?}", other),
                }
                assert!(items[1].adjacent);
            }
            other => panic!("expected Sequential, got {:?}", other),
        }
    }

    #[test]
    fn parse_nested_delimiters() {
        let d = parse_dialect("Test", "(|<Match>|)");
        match &d.rules[0] {
            Rule::Sequential(items) => {
                match &items[0].content {
                    ItemContent::Delimited { kind, inner } => {
                        assert_eq!(*kind, DelimKind::ParenPipe);
                        assert_eq!(inner.len(), 1);
                        match &inner[0].content {
                            ItemContent::DialectRef(name) => assert_eq!(name, "Match"),
                            other => panic!("expected DialectRef, got {:?}", other),
                        }
                    }
                    other => panic!("expected Delimited, got {:?}", other),
                }
            }
            other => panic!("expected Sequential, got {:?}", other),
        }
    }

    #[test]
    fn parse_adjacency_preserved() {
        let d = parse_dialect("Test", "@Type/@Variant");
        match &d.rules[0] {
            Rule::Sequential(items) => {
                assert_eq!(items.len(), 3);
                assert!(!items[0].adjacent); // first item never adjacent
                assert!(items[1].adjacent);  // / adjacent to @Type
                assert!(items[2].adjacent);  // @Variant adjacent to /
            }
            other => panic!("expected Sequential, got {:?}", other),
        }
    }

    #[test]
    fn parse_full_enum_synth() {
        let source = r#"
;; Enum.synth

// *@Variant
// *(@Variant <Type>)
// *{@Variant <Struct>}
// *(|@Enum <Enum>|)
// *{|@Struct <Struct>|}
"#;
        let d = parse_dialect("Enum", source);
        assert_eq!(d.name, "Enum");
        assert_eq!(d.rules.len(), 1);
        match &d.rules[0] {
            Rule::OrderedChoice(alts) => {
                assert_eq!(alts.len(), 5);
                // all are zero-or-more
                for alt in alts {
                    assert_eq!(alt.cardinality, Cardinality::ZeroOrMore);
                }
            }
            other => panic!("expected OrderedChoice, got {:?}", other),
        }
    }

    #[test]
    fn parse_full_root_synth() {
        let source = r#"
;; Root.synth

(@Module <Module>)
// *(@Enum <Enum>)
// *(@trait <TraitDecl>)
// *[@trait <TraitImpl>]
// *{@Struct <Struct>}
// *{|@Const <Type> @Literal|}
// *(|@Ffi <Ffi>|)
// ?[|<Process>|]
// *@Newtype <Type>
"#;
        let d = parse_dialect("Root", source);
        assert_eq!(d.name, "Root");
        assert_eq!(d.rules.len(), 2); // 1 sequential + 1 ordered choice
        match &d.rules[0] {
            Rule::Sequential(_) => {} // (@Module <Module>)
            other => panic!("expected Sequential, got {:?}", other),
        }
        match &d.rules[1] {
            Rule::OrderedChoice(alts) => {
                assert_eq!(alts.len(), 8); // 8 alternatives
            }
            other => panic!("expected OrderedChoice, got {:?}", other),
        }
    }
}
