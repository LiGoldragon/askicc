/// askicc — bootstrap compiler.
///
/// Reads .synth dialect files → populates aski-core domain-data-tree
/// → serializes as rkyv.

pub mod synth_token;
pub mod synth_lex;
#[cfg(test)]
mod synth_lex_tests;
pub mod synth_parse;
#[cfg(test)]
mod synth_parse_tests;
