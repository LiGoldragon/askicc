/// Synth lexer — byte scanner with adjacency tracking.
///
/// Resolves all strings to synth-core typed enums at lex time.
/// Space between tokens → adjacent=false. No space → adjacent=true.
/// Space adjacent to a delimiter is ignored at parse time; adjacency
/// between non-delimiter items still distinguishes required vs optional.
/// @ prefix → Declare. : prefix → Reference.
/// # prefix → Tag (closing # required).
/// ' prefix → Origin (names a place for lifetime tracking).

use synth_core::{
    Label, LabelKind, TagKind, Binding, Casing,
    DialectKind, SurfaceKind, LiteralToken, KeywordToken, DelimKind,
};
use crate::synth_token::{SynthToken, SynthSpanned};

pub struct SynthLexer<'a> {
    bytes: &'a [u8],
    pos: usize,
    last_end: usize,
}

impl<'a> SynthLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        SynthLexer { bytes: source.as_bytes(), pos: 0, last_end: 0 }
    }

    pub fn lex(mut self) -> Result<Vec<SynthSpanned>, String> {
        let mut tokens = Vec::new();
        while self.pos < self.bytes.len() {
            match self.bytes[self.pos] {
                b' ' | b'\t' | b'\n' | b'\r' => { self.pos += 1; }
                b';' if self.peek_at(1) == Some(b';') => self.skip_line(),
                b'#' => tokens.push(self.lex_tag()?),
                b'@' => tokens.push(self.lex_label(Binding::Declare)?),
                b':' if self.peek_at(1).map(|b| b.is_ascii_alphabetic()).unwrap_or(false) => {
                    tokens.push(self.lex_label(Binding::Reference)?)
                }
                b'\'' if self.peek_at(1).map(|b| b.is_ascii_alphabetic()).unwrap_or(false) => {
                    tokens.push(self.lex_label(Binding::Origin)?)
                }
                b'<' => tokens.push(self.lex_angle()?),
                b'_' => tokens.push(self.lex_literal_escape()?),
                b'"' => tokens.push(self.lex_string_lit()?),
                b'\'' => return Err(format!(
                    "bare ' at byte {} — origin sigil requires a PascalCase place name",
                    self.pos
                )),
                b'/' if self.peek_at(1) == Some(b'/') => tokens.push(self.emit_advance(SynthToken::Or, 2)),
                b'*' => tokens.push(self.emit_advance(SynthToken::ZeroOrMore, 1)),
                b'+' => tokens.push(self.emit_advance(SynthToken::OneOrMore, 1)),
                b'?' => tokens.push(self.emit_advance(SynthToken::Optional, 1)),
                b'(' if self.peek_at(1) == Some(b'|') => tokens.push(self.emit_open(DelimKind::ParenPipe, 2)),
                b'[' if self.peek_at(1) == Some(b'|') => tokens.push(self.emit_open(DelimKind::BracketPipe, 2)),
                b'{' if self.peek_at(1) == Some(b'|') => tokens.push(self.emit_open(DelimKind::BracePipe, 2)),
                b'|' if self.peek_at(1) == Some(b')') => tokens.push(self.emit_close(DelimKind::ParenPipe, 2)),
                b'|' if self.peek_at(1) == Some(b']') => tokens.push(self.emit_close(DelimKind::BracketPipe, 2)),
                b'|' if self.peek_at(1) == Some(b'}') => tokens.push(self.emit_close(DelimKind::BracePipe, 2)),
                b'(' => tokens.push(self.emit_open(DelimKind::Paren, 1)),
                b')' => tokens.push(self.emit_close(DelimKind::Paren, 1)),
                b'[' => tokens.push(self.emit_open(DelimKind::Bracket, 1)),
                b']' => tokens.push(self.emit_close(DelimKind::Bracket, 1)),
                b'{' => tokens.push(self.emit_open(DelimKind::Brace, 1)),
                b'}' => tokens.push(self.emit_close(DelimKind::Brace, 1)),
                b'A'..=b'Z' | b'a'..=b'z' => tokens.push(self.lex_bare_word()?),
                _ => tokens.push(self.lex_bare_operator()?),
            }
        }
        Ok(tokens)
    }

    // ── Helpers ──────────────────────────────────────────────

    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    fn skip_line(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
            self.pos += 1;
        }
    }

    fn adjacent(&self) -> bool {
        self.pos == self.last_end && self.last_end > 0
    }

    fn snap(&self) -> (bool, usize) {
        (self.adjacent(), self.pos)
    }

    fn emit(&mut self, token: SynthToken, adj: bool, pos: usize) -> SynthSpanned {
        SynthSpanned { token, adjacent: adj, pos }
    }

    fn emit_advance(&mut self, token: SynthToken, len: usize) -> SynthSpanned {
        let (adj, pos) = self.snap();
        self.pos += len;
        self.last_end = self.pos;
        self.emit(token, adj, pos)
    }

    fn emit_open(&mut self, kind: DelimKind, len: usize) -> SynthSpanned {
        self.emit_advance(SynthToken::Open(kind), len)
    }

    fn emit_close(&mut self, kind: DelimKind, len: usize) -> SynthSpanned {
        self.emit_advance(SynthToken::Close(kind), len)
    }

    fn read_word(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.bytes.len()
            && self.bytes[self.pos].is_ascii_alphanumeric()
        {
            self.pos += 1;
        }
        String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string()
    }

    // ── Token-specific lexers ───────────────────────────────

    fn lex_label(&mut self, binding: Binding) -> Result<SynthSpanned, String> {
        let (adj, pos) = self.snap();
        self.pos += 1; // skip @ or :
        let name = self.read_word();
        self.last_end = self.pos;
        let casing = if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            Casing::Pascal
        } else {
            Casing::Camel
        };
        let kind = Self::resolve_label_kind(&name)?;
        let label = Label { binding, kind, casing };
        Ok(self.emit(SynthToken::Label(label), adj, pos))
    }

    fn lex_tag(&mut self) -> Result<SynthSpanned, String> {
        let (adj, pos) = self.snap();
        self.pos += 1; // skip opening #
        let name = self.read_word();
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'#' {
            self.pos += 1; // skip closing #
        } else {
            return Err(format!("expected closing # after #{}", name));
        }
        self.last_end = self.pos;
        let kind = Self::resolve_tag_kind(&name)?;
        Ok(self.emit(SynthToken::Tag(kind), adj, pos))
    }

    fn lex_angle(&mut self) -> Result<SynthSpanned, String> {
        if self.peek_at(1) == Some(b'=') {
            return Ok(self.emit_advance(SynthToken::Literal(LiteralToken::LtEq), 2));
        }
        // Cross-surface reference: <:surface:Name>
        if self.peek_at(1) == Some(b':') {
            let (adj, pos) = self.snap();
            self.pos += 2; // skip <:
            let surface_name = self.read_word();
            if self.pos >= self.bytes.len() || self.bytes[self.pos] != b':' {
                return Err(format!("expected : after <:{}", surface_name));
            }
            self.pos += 1; // skip :
            let target_name = self.read_word();
            if self.pos >= self.bytes.len() || self.bytes[self.pos] != b'>' {
                return Err(format!("expected > after <:{}:{}", surface_name, target_name));
            }
            self.pos += 1; // skip >
            self.last_end = self.pos;
            let surface = Some(Self::resolve_surface_kind(&surface_name)?);
            let target = Self::resolve_dialect_kind(&target_name)?;
            return Ok(self.emit(SynthToken::DialectRef { surface, target }, adj, pos));
        }
        // Same-surface reference: <Name>
        if self.peek_at(1).map(|b| b.is_ascii_alphabetic()).unwrap_or(false) {
            let (adj, pos) = self.snap();
            self.pos += 1; // skip <
            let name = self.read_word();
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'>' {
                self.pos += 1;
            } else {
                return Err(format!("expected > after dialect ref <{}", name));
            }
            self.last_end = self.pos;
            let target = Self::resolve_dialect_kind(&name)?;
            return Ok(self.emit(SynthToken::DialectRef { surface: None, target }, adj, pos));
        }
        Ok(self.emit_advance(SynthToken::Literal(LiteralToken::Lt), 1))
    }

    fn lex_literal_escape(&mut self) -> Result<SynthSpanned, String> {
        let (adj, pos) = self.snap();
        self.pos += 1; // skip opening _
        let start = self.pos;
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'_' {
            self.pos += 1;
        }
        let content = String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string();
        if self.pos < self.bytes.len() {
            self.pos += 1; // skip closing _
        }
        self.last_end = self.pos;
        let token = Self::resolve_literal_escape(&content)?;
        Ok(self.emit(SynthToken::Literal(token), adj, pos))
    }

    fn lex_string_lit(&mut self) -> Result<SynthSpanned, String> {
        let (adj, pos) = self.snap();
        self.pos += 1; // skip opening "
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'"' {
            self.pos += 1;
        }
        if self.pos < self.bytes.len() {
            self.pos += 1; // skip closing "
        }
        self.last_end = self.pos;
        Ok(self.emit(SynthToken::StringLit, adj, pos))
    }

    fn lex_bare_word(&mut self) -> Result<SynthSpanned, String> {
        let (adj, pos) = self.snap();
        let name = self.read_word();
        self.last_end = self.pos;
        if let Some(kw) = Self::try_keyword(&name) {
            return Ok(self.emit(SynthToken::Keyword(kw), adj, pos));
        }
        Err(format!("unexpected bare word: {}", name))
    }

    fn lex_bare_operator(&mut self) -> Result<SynthSpanned, String> {
        let (adj, pos) = self.snap();
        let start = self.pos;

        if let Some(len) = self.try_multi_char_op() {
            self.pos += len;
            self.last_end = self.pos;
            let op = String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string();
            let token = Self::resolve_bare_operator(&op)?;
            return Ok(self.emit(SynthToken::Literal(token), adj, pos));
        }

        self.pos += 1;
        self.last_end = self.pos;
        let op = String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string();
        let token = Self::resolve_bare_operator(&op)?;
        Ok(self.emit(SynthToken::Literal(token), adj, pos))
    }

    fn try_multi_char_op(&self) -> Option<usize> {
        let remaining = &self.bytes[self.pos..];
        if remaining.starts_with(b"||") { return Some(2); }
        if remaining.starts_with(b"&&") { return Some(2); }
        if remaining.starts_with(b"==") { return Some(2); }
        if remaining.starts_with(b"!=") { return Some(2); }
        if remaining.starts_with(b">=") { return Some(2); }
        None
    }

    // ── Resolution tables ───────────────────────────────────

    pub fn resolve_filename(name: &str) -> Result<DialectKind, String> {
        Self::resolve_dialect_kind(name)
    }

    pub fn resolve_surface_kind(name: &str) -> Result<SurfaceKind, String> {
        match name {
            "core" | "Core" => Ok(SurfaceKind::Core),
            "aski" | "Aski" => Ok(SurfaceKind::Aski),
            "synth" | "Synth" => Ok(SurfaceKind::Synth),
            "exec" | "Exec" => Ok(SurfaceKind::Exec),
            "rfi" | "Rfi" => Ok(SurfaceKind::Rfi),
            other => Err(format!("unknown surface: {}", other)),
        }
    }

    fn resolve_label_kind(name: &str) -> Result<LabelKind, String> {
        // Case-insensitive match — casing is tracked separately.
        // Labels name the ROLE of a source-read identifier.
        let normalized = lowercase_first(name);
        match normalized.as_str() {
            // Name declarations — root-level data defs
            "moduleName" => Ok(LabelKind::ModuleName),
            "enumName" => Ok(LabelKind::EnumName),
            "structName" => Ok(LabelKind::StructName),
            "newtypeName" => Ok(LabelKind::NewtypeName),
            "constName" => Ok(LabelKind::ConstName),
            "rfiName" => Ok(LabelKind::RfiName),
            "traitName" => Ok(LabelKind::TraitName),
            "variantName" => Ok(LabelKind::VariantName),
            "fieldName" => Ok(LabelKind::FieldName),
            "methodName" => Ok(LabelKind::MethodName),
            "sigName" => Ok(LabelKind::SigName),
            "instanceName" => Ok(LabelKind::InstanceName),

            // Short-form declarations (no "Name" suffix)
            "type" => Ok(LabelKind::Type_),
            "param" => Ok(LabelKind::Param),
            "binding" => Ok(LabelKind::Binding),
            "role" => Ok(LabelKind::Role),
            "item" => Ok(LabelKind::Item),
            "foreignFunction" => Ok(LabelKind::ForeignFunction),
            "associatedName" => Ok(LabelKind::AssociatedName),

            // References (typically via :Label)
            "instance" => Ok(LabelKind::Instance),
            "variant" => Ok(LabelKind::Variant),
            "literal" => Ok(LabelKind::Literal),
            "bound" => Ok(LabelKind::Bound),
            "module" => Ok(LabelKind::Module),
            "method" => Ok(LabelKind::Method),
            "statement" => Ok(LabelKind::Statement),
            "field" => Ok(LabelKind::Field),
            "struct" => Ok(LabelKind::Struct),
            "constructor" => Ok(LabelKind::Constructor),
            "importedName" => Ok(LabelKind::ImportedName),

            // Origin / place labels
            "placeName" => Ok(LabelKind::PlaceName),

            // Synth-meta labels (synth surface describes itself)
            "labelName" => Ok(LabelKind::LabelName),
            "tagName" => Ok(LabelKind::TagName),
            "dialectName" => Ok(LabelKind::DialectName),

            _ => Err(format!("unknown label: {}", name)),
        }
    }

    fn resolve_tag_kind(name: &str) -> Result<TagKind, String> {
        // Tags identify the TYPE of an output node. Always PascalCase.
        match name {
            // Aski root
            "Module" => Ok(TagKind::Module),
            "Enum" => Ok(TagKind::Enum),
            "Struct" => Ok(TagKind::Struct),
            "Newtype" => Ok(TagKind::Newtype),
            "Const" => Ok(TagKind::Const),
            "Rfi" => Ok(TagKind::Rfi),
            "TraitDecl" => Ok(TagKind::TraitDecl),
            "TraitImpl" => Ok(TagKind::TraitImpl),
            "Program" => Ok(TagKind::Program),

            // Enum children
            "BareVariant" => Ok(TagKind::BareVariant),
            "DataVariant" => Ok(TagKind::DataVariant),
            "StructVariant" => Ok(TagKind::StructVariant),
            "NestedEnum" => Ok(TagKind::NestedEnum),
            "NestedStruct" => Ok(TagKind::NestedStruct),

            // Struct children
            "TypedField" => Ok(TagKind::TypedField),
            "SelfTypedField" => Ok(TagKind::SelfTypedField),

            // Module imports
            "Import" => Ok(TagKind::Import),

            // Statement variants (v0.19)
            "EarlyReturn" => Ok(TagKind::EarlyReturn),
            "WhileLoop" => Ok(TagKind::WhileLoop),
            "Iteration" => Ok(TagKind::Iteration),
            "MutationStmt" => Ok(TagKind::MutationStmt),
            "ExprStmt" => Ok(TagKind::ExprStmt),
            "LocalCanonical" => Ok(TagKind::LocalCanonical),
            "LocalTypeOnly" => Ok(TagKind::LocalTypeOnly),
            "LocalTypeInit" => Ok(TagKind::LocalTypeInit),
            "LocalConstruct" => Ok(TagKind::LocalConstruct),
            "LocalBind" => Ok(TagKind::LocalBind),

            // Expression variants
            "InstanceRef" => Ok(TagKind::InstanceRef),
            "PathVariant" => Ok(TagKind::PathVariant),
            "PathCall" => Ok(TagKind::PathCall),
            "LiteralExpr" => Ok(TagKind::LiteralExpr),
            "InlineEval" => Ok(TagKind::InlineEval),
            "MatchExpr" => Ok(TagKind::MatchExpr),
            "LoopExpr" => Ok(TagKind::LoopExpr),
            "IterExpr" => Ok(TagKind::IterExpr),
            "StructExpr" => Ok(TagKind::StructExpr),
            "BorrowExpr" => Ok(TagKind::BorrowExpr),
            "MutBorrowExpr" => Ok(TagKind::MutBorrowExpr),

            // Binary operators
            "BinOr" => Ok(TagKind::BinOr),
            "BinAnd" => Ok(TagKind::BinAnd),
            "BinEq" => Ok(TagKind::BinEq),
            "BinNotEq" => Ok(TagKind::BinNotEq),
            "BinLt" => Ok(TagKind::BinLt),
            "BinGt" => Ok(TagKind::BinGt),
            "BinLtEq" => Ok(TagKind::BinLtEq),
            "BinGtEq" => Ok(TagKind::BinGtEq),
            "BinAdd" => Ok(TagKind::BinAdd),
            "BinSub" => Ok(TagKind::BinSub),
            "BinMul" => Ok(TagKind::BinMul),
            "BinMod" => Ok(TagKind::BinMod),

            // Postfix operators
            "FieldAccess" => Ok(TagKind::FieldAccess),
            "MethodCall" => Ok(TagKind::MethodCall),
            "TryUnwrap" => Ok(TagKind::TryUnwrap),

            // Pattern variants
            "VariantBind" => Ok(TagKind::VariantBind),
            "VariantMatch" => Ok(TagKind::VariantMatch),
            "VariantAlt" => Ok(TagKind::VariantAlt),
            "StringMatch" => Ok(TagKind::StringMatch),

            // Param variants (v0.20 — newly tagged)
            "OwnedSelf" => Ok(TagKind::OwnedSelf),
            "BorrowedSelf" => Ok(TagKind::BorrowedSelf),
            "MutBorrowedSelf" => Ok(TagKind::MutBorrowedSelf),
            "OwnedNamed" => Ok(TagKind::OwnedNamed),
            "BorrowedNamed" => Ok(TagKind::BorrowedNamed),
            "MutBorrowedNamed" => Ok(TagKind::MutBorrowedNamed),
            "BareNamed" => Ok(TagKind::BareNamed),

            // Method-body variants
            "BlockBody" => Ok(TagKind::BlockBody),
            "MatchBody" => Ok(TagKind::MatchBody),
            "LoopBody" => Ok(TagKind::LoopBody),
            "IterBody" => Ok(TagKind::IterBody),
            "StructBody" => Ok(TagKind::StructBody),

            // Type-expression variants
            "Named" => Ok(TagKind::Named),
            "AppliedType" => Ok(TagKind::AppliedType),
            "GenericParamType" => Ok(TagKind::GenericParamType),
            "BorrowedType" => Ok(TagKind::BorrowedType),
            "MutBorrowedType" => Ok(TagKind::MutBorrowedType),
            "SelfAssocType" => Ok(TagKind::SelfAssocType),

            // Trait items (v0.20)
            "AssociatedType" => Ok(TagKind::AssociatedType),
            "AssociatedTypeImpl" => Ok(TagKind::AssociatedTypeImpl),

            // Self expression (v0.20)
            "SelfRef" => Ok(TagKind::SelfRef),

            // RFI surface (v0.20)
            "RfiGroup" => Ok(TagKind::RfiGroup),

            // Misc leaf constructs
            "TypeAnnotation" => Ok(TagKind::TypeAnnotation),
            "FieldInit" => Ok(TagKind::FieldInit),
            "MatchArm" => Ok(TagKind::MatchArm),
            "RfiFunction" => Ok(TagKind::RfiFunction),
            "BoundedParam" => Ok(TagKind::BoundedParam),
            "CallArgs" => Ok(TagKind::CallArgs),

            // Origin / lifetime annotations
            "PlaceRef" => Ok(TagKind::PlaceRef),
            "PlacePath" => Ok(TagKind::PlacePath),
            "PlaceUnion" => Ok(TagKind::PlaceUnion),
            "ViewType" => Ok(TagKind::ViewType),

            // Synth meta-tags
            "Sequential" => Ok(TagKind::Sequential),
            "OrderedChoice" => Ok(TagKind::OrderedChoice),
            "NamedItem" => Ok(TagKind::NamedItem),
            "TaggedItem" => Ok(TagKind::TaggedItem),
            "DialectRefItem" => Ok(TagKind::DialectRefItem),
            "DelimitedItem" => Ok(TagKind::DelimitedItem),
            "RepeatItem" => Ok(TagKind::RepeatItem),
            "LiteralItem" => Ok(TagKind::LiteralItem),
            "KeywordItem" => Ok(TagKind::KeywordItem),
            "DeclarePascal" => Ok(TagKind::DeclarePascal),
            "DeclareCamel" => Ok(TagKind::DeclareCamel),
            "ReferencePascal" => Ok(TagKind::ReferencePascal),
            "ReferenceCamel" => Ok(TagKind::ReferenceCamel),
            "OriginPascal" => Ok(TagKind::OriginPascal),
            "ZeroOrMore" => Ok(TagKind::ZeroOrMore),
            "OneOrMore" => Ok(TagKind::OneOrMore),
            "Optional" => Ok(TagKind::Optional),

            other => Err(format!("unknown tag: #{}#", other)),
        }
    }

    fn resolve_dialect_kind(name: &str) -> Result<DialectKind, String> {
        match name {
            "Root" => Ok(DialectKind::Root),
            "Module" => Ok(DialectKind::Module),
            "Enum" => Ok(DialectKind::Enum),
            "Struct" => Ok(DialectKind::Struct),
            "Body" => Ok(DialectKind::Body),
            "Expr" => Ok(DialectKind::Expr),
            "ExprOr" => Ok(DialectKind::ExprOr),
            "ExprAnd" => Ok(DialectKind::ExprAnd),
            "ExprCompare" => Ok(DialectKind::ExprCompare),
            "ExprAdd" => Ok(DialectKind::ExprAdd),
            "ExprMul" => Ok(DialectKind::ExprMul),
            "ExprPostfix" => Ok(DialectKind::ExprPostfix),
            "ExprAtom" => Ok(DialectKind::ExprAtom),
            "Type" => Ok(DialectKind::Type_),
            "TypeApplication" => Ok(DialectKind::TypeApplication),
            "GenericParam" => Ok(DialectKind::GenericParam),
            "Statement" => Ok(DialectKind::Statement),
            "Mutation" => Ok(DialectKind::Mutation),
            "Param" => Ok(DialectKind::Param),
            "Signature" => Ok(DialectKind::Signature),
            "Method" => Ok(DialectKind::Method),
            "MethodBody" => Ok(DialectKind::MethodBody),
            "TraitItem" => Ok(DialectKind::TraitItem),
            "TraitImplItem" => Ok(DialectKind::TraitImplItem),
            "Match" => Ok(DialectKind::Match),
            "Pattern" => Ok(DialectKind::Pattern),
            "Loop" => Ok(DialectKind::Loop),
            "IterationSource" => Ok(DialectKind::IterationSource),
            "StructConstruct" => Ok(DialectKind::StructConstruct),
            "Rfi" => Ok(DialectKind::Rfi),
            "Origin" => Ok(DialectKind::Origin),
            "FieldPath" => Ok(DialectKind::FieldPath),
            "ViewType" => Ok(DialectKind::ViewType),
            "Program" => Ok(DialectKind::Program),
            "SynthRule" => Ok(DialectKind::SynthRule),
            "SynthAlt" => Ok(DialectKind::SynthAlt),
            "SynthItem" => Ok(DialectKind::SynthItem),
            "SynthLabel" => Ok(DialectKind::SynthLabel),
            "SynthCard" => Ok(DialectKind::SynthCard),
            other => Err(format!("unknown dialect: <{}>", other)),
        }
    }

    fn resolve_literal_escape(content: &str) -> Result<LiteralToken, String> {
        match content {
            "@" => Ok(LiteralToken::At),
            "~" => Ok(LiteralToken::Tilde),
            "$" => Ok(LiteralToken::Dollar),
            "*" => Ok(LiteralToken::Star),
            "+" => Ok(LiteralToken::Plus),
            "?" => Ok(LiteralToken::Question),
            "&" => Ok(LiteralToken::Ampersand),
            "//" => Ok(LiteralToken::DoubleSlash),
            "#" => Ok(LiteralToken::Hash),
            "<" => Ok(LiteralToken::Lt),
            ">" => Ok(LiteralToken::Gt),
            ":" => Ok(LiteralToken::Colon),
            "'" => Ok(LiteralToken::Apostrophe),
            other => Err(format!("unknown literal escape: _{}_", other)),
        }
    }

    fn resolve_bare_operator(op: &str) -> Result<LiteralToken, String> {
        match op {
            "==" => Ok(LiteralToken::Eq),
            "!=" => Ok(LiteralToken::NotEq),
            "<" => Ok(LiteralToken::Lt),
            ">" => Ok(LiteralToken::Gt),
            "<=" => Ok(LiteralToken::LtEq),
            ">=" => Ok(LiteralToken::GtEq),
            "||" => Ok(LiteralToken::LogicalOr),
            "&&" => Ok(LiteralToken::LogicalAnd),
            "-" => Ok(LiteralToken::Minus),
            "%" => Ok(LiteralToken::Percent),
            "." => Ok(LiteralToken::Dot),
            "/" => Ok(LiteralToken::Slash),
            "^" => Ok(LiteralToken::Caret),
            ":" => Ok(LiteralToken::Colon),
            "|" => Ok(LiteralToken::Pipe),
            "~" => Ok(LiteralToken::Tilde),
            other => Err(format!("unknown operator: {}", other)),
        }
    }

    fn try_keyword(name: &str) -> Option<KeywordToken> {
        match name {
            "self" => Some(KeywordToken::Self_),
            _ => None,
        }
    }
}

fn lowercase_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_ascii_lowercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}
