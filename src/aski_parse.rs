/// Aski parser — full .aski declaration surface → domain tree.
///
/// Uses the Logos lexer. Handles all v0.17 constructs.
/// Types mirror the definitions in askicc/aski/*.aski.

use crate::lexer::{Token, Spanned, lex};

/// A parsed .aski module.
#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub exports: Vec<String>,
    pub domains: Vec<Domain>,
}

/// Any domain definition — enum, struct, or newtype.
#[derive(Debug)]
pub enum Domain {
    Enum(EnumDef),
    Struct(StructDef),
    Newtype(NewtypeDef),
}

#[derive(Debug)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug)]
pub enum EnumVariant {
    /// Bare: Fire, Earth
    Bare(String),
    /// Data-carrying: (Some $Value), (Int I64)
    Data { name: String, payload: TypeExpr },
    /// Struct variant: {BinAdd (Left [Box Expr]) (Right [Box Expr]) Span}
    Struct(StructDef),
}

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub enum StructField {
    Typed { name: String, typ: TypeExpr },
    SelfTyped(String),
}

#[derive(Debug)]
pub struct NewtypeDef {
    pub name: String,
    pub wraps: TypeExpr,
}

/// Type expression — handles simple, application, and nested.
#[derive(Debug)]
pub enum TypeExpr {
    /// Simple: U32, String, Expr, FieldName
    Simple(String),
    /// Application: [Vec Item], [Box Expr], [Option [Box Expr]]
    Application { constructor: String, args: Vec<TypeExpr> },
}

impl Module {
    pub fn parse(source: &str) -> Result<Self, String> {
        let tokens = lex(source).map_err(|errs| {
            errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
        })?;
        let mut parser = AskiParser::new(&tokens);
        parser.parse_module()
    }
}

struct AskiParser<'a> {
    tokens: &'a [Spanned],
    pos: usize,
}

impl<'a> AskiParser<'a> {
    fn new(tokens: &'a [Spanned]) -> Self {
        AskiParser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.skip_newlines_peek()
    }

    fn skip_newlines_peek(&self) -> Option<&Token> {
        let mut i = self.pos;
        while i < self.tokens.len() {
            if !matches!(self.tokens[i].token, Token::Newline) {
                return Some(&self.tokens[i].token);
            }
            i += 1;
        }
        None
    }

    fn advance(&mut self) -> Option<&Token> {
        self.skip_newlines();
        let tok = self.tokens.get(self.pos).map(|t| &t.token);
        if tok.is_some() { self.pos += 1; }
        tok
    }

    fn skip_newlines(&mut self) {
        while self.pos < self.tokens.len() && matches!(self.tokens[self.pos].token, Token::Newline) {
            self.pos += 1;
        }
    }

    fn at_end(&self) -> bool {
        let mut i = self.pos;
        while i < self.tokens.len() {
            if !matches!(self.tokens[i].token, Token::Newline) {
                return false;
            }
            i += 1;
        }
        true
    }

    fn expect_pascal(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token::PascalIdent(s)) => Ok(s.clone()),
            other => Err(format!("expected PascalCase, got {:?}", other)),
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let got = self.advance();
        if got == Some(expected) { Ok(()) }
        else { Err(format!("expected {:?}, got {:?}", expected, got)) }
    }

    fn parse_module(&mut self) -> Result<Module, String> {
        let mut module_name = None;
        let mut module_exports = Vec::new();
        let mut domains = Vec::new();

        while !self.at_end() {
            let before = self.pos;
            match self.peek() {
                Some(Token::LParen) => {
                    let enum_def = self.parse_enum()?;
                    if module_name.is_none() {
                        module_name = Some(enum_def.name.clone());
                        for v in &enum_def.variants {
                            if let EnumVariant::Bare(name) = v {
                                module_exports.push(name.clone());
                            }
                        }
                    } else {
                        domains.push(Domain::Enum(enum_def));
                    }
                }
                Some(Token::LBrace) => {
                    let struct_def = self.parse_struct()?;
                    domains.push(Domain::Struct(struct_def));
                }
                Some(Token::PascalIdent(_)) => {
                    let newtype_def = self.parse_newtype()?;
                    domains.push(Domain::Newtype(newtype_def));
                }
                other => {
                    return Err(format!("expected (, {{ or PascalCase at root, got {:?}", other));
                }
            }
            if self.pos <= before {
                return Err(format!("parser stuck at root position {}", self.pos));
            }
        }

        let name = module_name.ok_or("no module declaration found")?;
        Ok(Module { name, exports: module_exports, domains })
    }

    fn parse_enum(&mut self) -> Result<EnumDef, String> {
        self.expect(&Token::LParen)?;
        let name = self.expect_pascal()?;
        let mut variants = Vec::new();

        while self.peek() != Some(&Token::RParen) {
            if self.at_end() {
                return Err("unexpected EOF inside enum".into());
            }
            let before = self.pos;
            let variant = self.parse_enum_variant()?;
            if self.pos <= before {
                return Err(format!("parser stuck at position {} in enum {}", self.pos, name));
            }
            variants.push(variant);
        }

        self.expect(&Token::RParen)?;
        Ok(EnumDef { name, variants })
    }

    fn parse_enum_variant(&mut self) -> Result<EnumVariant, String> {
        match self.peek() {
            // data-carrying variant: (Name Type)
            Some(Token::LParen) => {
                self.expect(&Token::LParen)?;
                let name = self.expect_pascal()?;
                if self.peek() == Some(&Token::RParen) {
                    // empty parens — just a variant name in parens? unlikely but handle
                    self.expect(&Token::RParen)?;
                    Ok(EnumVariant::Bare(name))
                } else {
                    let payload = self.parse_type_expr()?;
                    self.expect(&Token::RParen)?;
                    Ok(EnumVariant::Data { name, payload })
                }
            }
            // struct variant: {Name (Field Type) ...}
            Some(Token::LBrace) => {
                let struct_def = self.parse_struct()?;
                Ok(EnumVariant::Struct(struct_def))
            }
            // bare variant: Fire, Earth
            Some(Token::PascalIdent(_)) => {
                let name = self.expect_pascal()?;
                Ok(EnumVariant::Bare(name))
            }
            other => {
                Err(format!("expected enum variant, got {:?}", other))
            }
        }
    }

    fn parse_struct(&mut self) -> Result<StructDef, String> {
        self.expect(&Token::LBrace)?;
        let name = self.expect_pascal()?;
        let mut fields = Vec::new();

        while self.peek() != Some(&Token::RBrace) {
            if self.at_end() {
                return Err("unexpected EOF inside struct".into());
            }
            let before = self.pos;
            let field = self.parse_struct_field()?;
            if self.pos <= before {
                return Err(format!("parser stuck at position {} in struct {}", self.pos, name));
            }
            fields.push(field);
        }

        self.expect(&Token::RBrace)?;
        Ok(StructDef { name, fields })
    }

    fn parse_struct_field(&mut self) -> Result<StructField, String> {
        match self.peek() {
            // typed field: (Name Type)
            Some(Token::LParen) => {
                self.expect(&Token::LParen)?;
                let name = self.expect_pascal()?;
                let typ = self.parse_type_expr()?;
                self.expect(&Token::RParen)?;
                Ok(StructField::Typed { name, typ })
            }
            // self-typed field: Name
            Some(Token::PascalIdent(_)) => {
                let name = self.expect_pascal()?;
                Ok(StructField::SelfTyped(name))
            }
            other => {
                Err(format!("expected struct field, got {:?}", other))
            }
        }
    }

    fn parse_newtype(&mut self) -> Result<NewtypeDef, String> {
        let name = self.expect_pascal()?;
        let wraps = self.parse_type_expr()?;
        Ok(NewtypeDef { name, wraps })
    }

    fn parse_type_expr(&mut self) -> Result<TypeExpr, String> {
        match self.peek() {
            // type application: [Vec Item], [Option [Box Expr]]
            Some(Token::LBracket) => {
                self.expect(&Token::LBracket)?;
                let constructor = self.expect_pascal()?;
                let mut args = Vec::new();
                while self.peek() != Some(&Token::RBracket) {
                    if self.at_end() {
                        return Err("unexpected EOF inside type application".into());
                    }
                    let before = self.pos;
                    let arg = self.parse_type_expr()?;
                    if self.pos <= before {
                        return Err(format!("parser stuck at position {} in type application", self.pos));
                    }
                    args.push(arg);
                }
                self.expect(&Token::RBracket)?;
                Ok(TypeExpr::Application { constructor, args })
            }
            // simple type: PascalCase
            Some(Token::PascalIdent(_)) => {
                let name = self.expect_pascal()?;
                Ok(TypeExpr::Simple(name))
            }
            other => {
                Err(format!("expected type expression, got {:?}", other))
            }
        }
    }
}
