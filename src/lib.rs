/// askicc — bootstrap compiler.
///
/// Reads .synth files from source/<surface>/ directories, produces
/// synth-core dsl-tree (Dialect, Rule, Item, ...) and serializes as
/// a single rkyv `dsls.rkyv` with all four DSLs flattened.

pub mod synth_token;
pub mod synth_lex;
#[cfg(test)]
mod synth_lex_tests;
pub mod synth_parse;
#[cfg(test)]
mod synth_parse_tests;
