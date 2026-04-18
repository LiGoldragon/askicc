/// Synth token types — resolved to aski-core typed enums at lex time.

use synth_core::{Label, TagKind, DialectKind, LiteralToken, KeywordToken, DelimKind, SurfaceKind};

#[derive(Debug, Clone, PartialEq)]
pub enum SynthToken {
    Or,                              // // (ordered choice separator)
    OneOrMore,                       // + (synth cardinality)
    ZeroOrMore,                      // * (synth cardinality)
    Optional,                        // ? (synth cardinality)
    Label(Label),                    // @Label (declare) or :Label (reference)
    Tag(TagKind),                    // #Name# — names the output variant without matching
    Keyword(KeywordToken),           // bare keyword (Self)
    DialectRef {                     // <Name> or <:surface:Name>
        surface: Option<SurfaceKind>,
        target: DialectKind,
    },
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
