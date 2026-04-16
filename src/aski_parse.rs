/// Aski parser — full .aski declaration surface → domain tree.
///
/// Uses the Logos lexer. Handles all v0.17 constructs:
/// modules, enums, structs, newtypes, data-carrying variants,
/// struct variants, type application, generics, nested definitions,
/// traits, FFI, constants.

use crate::lexer::{Token, Spanned, lex};

// TODO: define domain tree types and implement parsing
// This will mirror the types in askicc/aski/*.aski
pub struct AskiFile {
    pub path: String,
}

impl AskiFile {
    pub fn parse(_path: &str, _source: &str) -> Result<Self, String> {
        Err("aski parser not yet implemented".into())
    }
}
