/// Synth token types — resolved to aski-core typed enums at lex time.

use aski_core::{DeclareLabel, DialectKind, LiteralToken, DelimKind};

#[derive(Debug, Clone, PartialEq)]
pub enum SynthToken {
    Or,                              // // (ordered choice separator)
    OneOrMore,                       // + (synth cardinality)
    ZeroOrMore,                      // * (synth cardinality)
    Optional,                        // ? (synth cardinality)
    Declare(DeclareLabel),           // @Label — resolved at lex time
    DialectRef(DialectKind),         // <Name> — resolved at lex time
    Literal(LiteralToken),           // _X_ or bare operator — resolved at lex time
    BareIdent(DeclareLabel),         // bare PascalCase/camelCase (e.g. Self after _@_)
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
