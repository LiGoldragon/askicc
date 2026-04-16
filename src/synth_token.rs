/// Synth token types — resolved to aski-core typed enums at lex time.

use aski_core::{Label, LabelKind, Binding, Casing, DialectKind, LiteralToken, KeywordToken, DelimKind};

#[derive(Debug, Clone, PartialEq)]
pub enum SynthToken {
    Or,                              // // (ordered choice separator)
    OneOrMore,                       // + (synth cardinality)
    ZeroOrMore,                      // * (synth cardinality)
    Optional,                        // ? (synth cardinality)
    Label(Label),                    // @Label (declare) or :Label (reference)
    Keyword(KeywordToken),           // bare keyword (Self, Main)
    DialectRef(DialectKind),         // <Name> — resolved at lex time
    Literal(LiteralToken),           // _X_ or bare operator — resolved at lex time
    StringLit,                       // "literal"
    Open(DelimKind),                 // ( [ { (| [| {|
    Close(DelimKind),                // ) ] } |) |] |}
}

#[derive(Debug, Clone)]
pub struct SynthSpanned {
    pub token: SynthToken,
    pub adjacent: bool,
    pub pos: usize,
}
