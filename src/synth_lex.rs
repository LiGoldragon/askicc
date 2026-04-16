/// Synth lexer — byte scanner with adjacency tracking.
///
/// Resolves all strings to aski-core typed enums at lex time.
/// Space between tokens → adjacent=false. No space → adjacent=true.

use aski_core::{DeclareLabel, DialectKind, LiteralToken, DelimKind};
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
                b'@' => tokens.push(self.lex_declare()?),
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

    fn spanned(&mut self, token: SynthToken) -> SynthSpanned {
        let adj = self.adjacent();
        let p = self.pos;
        SynthSpanned { token, adjacent: adj, pos: p }
    }

    fn emit_advance(&mut self, token: SynthToken, len: usize) -> SynthSpanned {
        let s = self.spanned(token);
        self.pos += len;
        self.last_end = self.pos;
        s
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
            && (self.bytes[self.pos].is_ascii_alphanumeric() || self.bytes[self.pos] == b'_')
        {
            self.pos += 1;
        }
        String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string()
    }

    // ── Token-specific lexers ───────────────────────────────

    fn lex_declare(&mut self) -> Result<SynthSpanned, String> {
        let s = self.spanned(SynthToken::Or); // placeholder, overwritten below
        self.pos += 1; // skip @
        let name = self.read_word();
        self.last_end = self.pos;
        let label = Self::resolve_declare_label(&name)?;
        Ok(SynthSpanned { token: SynthToken::Declare(label), adjacent: s.adjacent, pos: s.pos })
    }

    fn lex_angle(&mut self) -> Result<SynthSpanned, String> {
        // Check for <= or >= first
        if self.peek_at(1) == Some(b'=') {
            return Ok(self.emit_advance(SynthToken::Literal(LiteralToken::LtEq), 2));
        }
        // Check for <alphabetic> → dialect ref
        if self.peek_at(1).map(|b| b.is_ascii_alphabetic()).unwrap_or(false) {
            let s = self.spanned(SynthToken::Or); // placeholder
            self.pos += 1; // skip <
            let name = self.read_word();
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'>' {
                self.pos += 1;
            } else {
                return Err(format!("expected > after dialect ref <{}", name));
            }
            self.last_end = self.pos;
            let kind = Self::resolve_dialect_kind(&name)?;
            return Ok(SynthSpanned { token: SynthToken::DialectRef(kind), adjacent: s.adjacent, pos: s.pos });
        }
        // Bare < operator
        Ok(self.emit_advance(SynthToken::Literal(LiteralToken::Lt), 1))
    }

    fn lex_literal_escape(&mut self) -> Result<SynthSpanned, String> {
        let s = self.spanned(SynthToken::Or); // placeholder
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
        Ok(SynthSpanned { token: SynthToken::Literal(token), adjacent: s.adjacent, pos: s.pos })
    }

    fn lex_string_lit(&mut self) -> Result<SynthSpanned, String> {
        let s = self.spanned(SynthToken::Or); // placeholder
        self.pos += 1; // skip opening "
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'"' {
            self.pos += 1;
        }
        if self.pos < self.bytes.len() {
            self.pos += 1; // skip closing "
        }
        self.last_end = self.pos;
        Ok(SynthSpanned { token: SynthToken::StringLit, adjacent: s.adjacent, pos: s.pos })
    }

    fn lex_bare_word(&mut self) -> Result<SynthSpanned, String> {
        let s = self.spanned(SynthToken::Or); // placeholder
        let name = self.read_word();
        self.last_end = self.pos;
        // Bare identifiers after literal escapes (e.g. Self after _@_)
        let label = Self::resolve_declare_label(&name)?;
        Ok(SynthSpanned { token: SynthToken::BareIdent(label), adjacent: s.adjacent, pos: s.pos })
    }

    fn lex_bare_operator(&mut self) -> Result<SynthSpanned, String> {
        let s = self.spanned(SynthToken::Or); // placeholder
        let start = self.pos;

        // Multi-char operators: check longest match first
        if let Some(len) = self.try_multi_char_op() {
            self.pos += len;
            self.last_end = self.pos;
            let op = String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string();
            let token = Self::resolve_bare_operator(&op)?;
            return Ok(SynthSpanned { token: SynthToken::Literal(token), adjacent: s.adjacent, pos: s.pos });
        }

        // Single char operator
        self.pos += 1;
        // Stop at alphabetic (don't consume :Module as one token)
        self.last_end = self.pos;
        let op = String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string();
        let token = Self::resolve_bare_operator(&op)?;
        Ok(SynthSpanned { token: SynthToken::Literal(token), adjacent: s.adjacent, pos: s.pos })
    }

    fn try_multi_char_op(&self) -> Option<usize> {
        let remaining = &self.bytes[self.pos..];
        if remaining.starts_with(b"||") { return Some(2); }
        if remaining.starts_with(b"&&") { return Some(2); }
        if remaining.starts_with(b"==") { return Some(2); }
        if remaining.starts_with(b"!=") { return Some(2); }
        if remaining.starts_with(b">=") { return Some(2); }
        // Note: <= handled in lex_angle, < handled there too
        None
    }

    // ── String → Variant resolution tables ──────────────────

    /// Map .synth filename (without extension) to DialectKind.
    pub fn resolve_filename(name: &str) -> Result<DialectKind, String> {
        Self::resolve_dialect_kind(name)
    }

    fn resolve_declare_label(name: &str) -> Result<DeclareLabel, String> {
        match name {
            "Module" => Ok(DeclareLabel::Module),
            "Enum" => Ok(DeclareLabel::Enum),
            "Struct" => Ok(DeclareLabel::Struct),
            "Type" => Ok(DeclareLabel::Type_),
            "Newtype" => Ok(DeclareLabel::Newtype),
            "Constructor" => Ok(DeclareLabel::Constructor),
            "Variant" => Ok(DeclareLabel::Variant),
            "Field" => Ok(DeclareLabel::Field),
            "Literal" => Ok(DeclareLabel::Literal),
            "Self" => Ok(DeclareLabel::Self_),
            "Name" => Ok(DeclareLabel::Name),
            "Binding" => Ok(DeclareLabel::Binding),
            "Param" => Ok(DeclareLabel::Param),
            "Bound" => Ok(DeclareLabel::Bound),
            "Export" => Ok(DeclareLabel::Export),
            "Import" => Ok(DeclareLabel::Import),
            "Const" => Ok(DeclareLabel::Const),
            "Ffi" => Ok(DeclareLabel::Ffi),
            "trait" => Ok(DeclareLabel::Trait),
            "method" => Ok(DeclareLabel::Method),
            "foreignFunction" => Ok(DeclareLabel::ForeignFunction),
            "signature" => Ok(DeclareLabel::Signature),
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
}
