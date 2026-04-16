/// Codegen — domain tree → scoped Rust enums/structs.
///
/// Generates the enum-as-index architecture.
/// Also emits dialect structures for askic.

// TODO: define codegen types and implement
pub struct CodegenOutput {
    pub rust_source: String,
}
