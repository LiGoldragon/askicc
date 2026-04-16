/// Synth lexer — byte scanner with adjacency tracking.
///
/// Resolves all strings to aski-core typed enums at lex time.
/// Space between tokens → adjacent=false. No space → adjacent=true.
/// @ prefix → Declare. : prefix → Reference.

use aski_core::{Label, LabelKind, Binding, Casing, DialectKind, LiteralToken, KeywordToken, DelimKind};
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
                b'@' => tokens.push(self.lex_label(Binding::Declare)?),
                b':' if self.peek_at(1).map(|b| b.is_ascii_alphabetic()).unwrap_or(false) => {
                    tokens.push(self.lex_label(Binding::Reference)?)
                }
                b'<' => tokens.push(self.lex_angle()?),
                b'_' => tokens.push(self.lex_literal_escape()?),
                b'"' => tokens.push(self.lex_string_lit()?),
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

    fn lex_angle(&mut self) -> Result<SynthSpanned, String> {
        if self.peek_at(1) == Some(b'=') {
            return Ok(self.emit_advance(SynthToken::Literal(LiteralToken::LtEq), 2));
        }
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
            let kind = Self::resolve_dialect_kind(&name)?;
            return Ok(self.emit(SynthToken::DialectRef(kind), adj, pos));
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
        // Check keywords first
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

    fn resolve_label_kind(name: &str) -> Result<LabelKind, String> {
        // Case-insensitive match — casing is tracked separately
        match name {
            "Module" | "module" => Ok(LabelKind::Module),
            "Enum" | "enum" => Ok(LabelKind::Enum),
            "Struct" | "struct" => Ok(LabelKind::Struct),
            "Type" | "type" => Ok(LabelKind::Type_),
            "Newtype" | "newtype" => Ok(LabelKind::Newtype),
            "Constructor" | "constructor" => Ok(LabelKind::Constructor),
            "Variant" | "variant" => Ok(LabelKind::Variant),
            "Field" | "field" => Ok(LabelKind::Field),
            "Literal" | "literal" => Ok(LabelKind::Literal),
            "Instance" | "instance" => Ok(LabelKind::Instance),
            "Binding" | "binding" => Ok(LabelKind::Binding),
            "Param" | "param" => Ok(LabelKind::Param),
            "Bound" | "bound" => Ok(LabelKind::Bound),
            "Const" | "const" => Ok(LabelKind::Const),
            "Ffi" | "ffi" => Ok(LabelKind::Ffi),
            "Trait" | "trait" => Ok(LabelKind::Trait),
            "Method" | "method" => Ok(LabelKind::Method),
            "ForeignFunction" | "foreignFunction" => Ok(LabelKind::ForeignFunction),
            "Signature" | "signature" => Ok(LabelKind::Signature),
            "Unknown" | "unknown" => Ok(LabelKind::Unknown),
            "ObjectExport" | "objectExport" => Ok(LabelKind::ObjectExport),
            "ActionExport" | "actionExport" => Ok(LabelKind::ActionExport),
            "ObjectImport" | "objectImport" => Ok(LabelKind::ObjectImport),
            "ActionImport" | "actionImport" => Ok(LabelKind::ActionImport),
            other => Err(format!("unknown label: {}", other)),
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
            "Instance" => Ok(DialectKind::Instance),
            "Mutation" => Ok(DialectKind::Mutation),
            "Param" => Ok(DialectKind::Param),
            "Signature" => Ok(DialectKind::Signature),
            "Method" => Ok(DialectKind::Method),
            "TraitDecl" => Ok(DialectKind::TraitDecl),
            "TraitImpl" => Ok(DialectKind::TraitImpl),
            "TypeImpl" => Ok(DialectKind::TypeImpl),
            "Match" => Ok(DialectKind::Match),
            "Pattern" => Ok(DialectKind::Pattern),
            "Loop" => Ok(DialectKind::Loop),
            "Process" => Ok(DialectKind::Process),
            "StructConstruct" => Ok(DialectKind::StructConstruct),
            "Ffi" => Ok(DialectKind::Ffi),
            other => Err(format!("unknown dialect: <{}>", other)),
        }
    }

    fn resolve_literal_escape(content: &str) -> Result<LiteralToken, String> {
        match content {
            "@" => Ok(LiteralToken::At),
            "~@" => Ok(LiteralToken::MutAt),
            ":@" => Ok(LiteralToken::BorrowAt),
            "$" => Ok(LiteralToken::Dollar),
            "*" => Ok(LiteralToken::Star),
            "+" => Ok(LiteralToken::Plus),
            "?" => Ok(LiteralToken::Question),
            "&" => Ok(LiteralToken::Ampersand),
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
            other => Err(format!("unknown operator: {}", other)),
        }
    }

    fn try_keyword(name: &str) -> Option<KeywordToken> {
        match name {
            "Self" => Some(KeywordToken::Self_),
            "Main" => Some(KeywordToken::Main),
            _ => None,
        }
    }
}
