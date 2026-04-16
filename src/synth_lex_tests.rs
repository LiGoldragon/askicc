#[cfg(test)]
mod tests {
    use crate::synth_lex::*;

    #[test]
    fn lex_simple_rule() {
        let tokens = synth_lex("@Module <Module>").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, SynthToken::Declare("Module".into()));
        assert_eq!(tokens[1].token, SynthToken::DialectRef("Module".into()));
        assert!(!tokens[1].adjacent); // space between
    }

    #[test]
    fn lex_ordered_choice() {
        let tokens = synth_lex("// *@Variant").unwrap();
        assert_eq!(tokens[0].token, SynthToken::Or);
        assert_eq!(tokens[1].token, SynthToken::ZeroOrMore);
        assert_eq!(tokens[2].token, SynthToken::Declare("Variant".into()));
    }

    #[test]
    fn lex_delimited() {
        let tokens = synth_lex("(@Enum <Enum>)").unwrap();
        assert_eq!(tokens[0].token, SynthToken::LParen);
        assert_eq!(tokens[1].token, SynthToken::Declare("Enum".into()));
        assert_eq!(tokens[1].adjacent, true);
        assert_eq!(tokens[2].token, SynthToken::DialectRef("Enum".into()));
        assert_eq!(tokens[3].token, SynthToken::RParen);
    }

    #[test]
    fn lex_literal_escape() {
        let tokens = synth_lex("_@_@Name").unwrap();
        assert_eq!(tokens[0].token, SynthToken::LiteralEscape("@".into()));
        assert_eq!(tokens[1].token, SynthToken::Declare("Name".into()));
        assert_eq!(tokens[1].adjacent, true);
    }

    #[test]
    fn lex_piped_delimiters() {
        let tokens = synth_lex("(|<Match>|)").unwrap();
        assert_eq!(tokens[0].token, SynthToken::LParenPipe);
        assert_eq!(tokens[1].token, SynthToken::DialectRef("Match".into()));
        assert_eq!(tokens[1].adjacent, true);
        assert_eq!(tokens[2].token, SynthToken::RParenPipe);
    }

    #[test]
    fn lex_adjacency() {
        let tokens = synth_lex("@Type/@Variant").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, SynthToken::Declare("Type".into()));
        assert_eq!(tokens[1].token, SynthToken::Ident("/".into()));
        assert_eq!(tokens[1].adjacent, true);
        assert_eq!(tokens[2].token, SynthToken::Declare("Variant".into()));
        assert_eq!(tokens[2].adjacent, true);
    }

    #[test]
    fn lex_spaced_vs_adjacent() {
        let tokens = synth_lex("@Type / @Variant").unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(!tokens[1].adjacent); // space before /
        assert!(!tokens[2].adjacent); // space before @Variant
    }

    #[test]
    fn lex_comment() {
        let tokens = synth_lex(";; comment\n@Name").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, SynthToken::Declare("Name".into()));
    }

    #[test]
    fn lex_bare_operator() {
        let tokens = synth_lex("<ExprAnd> || <ExprOr>").unwrap();
        assert_eq!(tokens[0].token, SynthToken::DialectRef("ExprAnd".into()));
        assert_eq!(tokens[1].token, SynthToken::Ident("||".into()));
        assert_eq!(tokens[2].token, SynthToken::DialectRef("ExprOr".into()));
    }

    #[test]
    fn lex_full_root_synth() {
        let source = r#"
;; Root.synth — root dialect

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
        let tokens = synth_lex(source).unwrap();
        // should parse without errors
        assert!(tokens.len() > 20);
    }

    #[test]
    fn lex_expr_compare_synth() {
        let source = std::fs::read_to_string("source/ExprCompare.synth").unwrap();
        let tokens = synth_lex(&source).unwrap();
        eprintln!("ExprCompare tokens: {}", tokens.len());
        for t in &tokens {
            eprintln!("  {:?} adj={}", t.token, t.adjacent);
        }
        assert!(tokens.len() > 0);
    }

    #[test]
    fn lex_all_synth_files() {
        for entry in std::fs::read_dir("source").unwrap() {
            let path = entry.unwrap().path();
            if path.extension().map(|e| e == "synth").unwrap_or(false) {
                let source = std::fs::read_to_string(&path).unwrap();
                let result = synth_lex(&source);
                assert!(result.is_ok(), "failed to lex {}: {:?}", path.display(), result.err());
            }
        }
    }
}
