/// Synth parser — .synth tokens → Dialect domains.

use crate::synth_lex::{SynthToken, SynthSpanned};

#[derive(Debug, Clone)]
pub struct Dialect {
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    Sequential(Vec<Item>),
    OrderedChoice(Vec<Alternative>),
}

#[derive(Debug, Clone)]
pub struct Alternative {
    pub items: Vec<Item>,
    pub cardinality: Cardinality,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub content: ItemContent,
    pub adjacent: bool,
}

#[derive(Debug, Clone)]
pub enum ItemContent {
    Declare { casing: Casing, label: String },
    DialectRef(String),
    Delimited { kind: DelimKind, inner: Vec<Item> },
    Literal(String),
    Repeat { kind: Cardinality, inner: Box<Item> },
    LiteralValue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cardinality { One, ZeroOrMore, OneOrMore, Optional }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Casing { Pascal, Camel }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DelimKind { Paren, Bracket, Brace, ParenPipe, BracketPipe, BracePipe }

impl Dialect {
    pub fn parse(name: &str, tokens: &[SynthSpanned]) -> Result<Self, String> {
        let mut parser = SynthParser::new(tokens);
        let rules = parser.parse_rules()?;
        Ok(Dialect { name: name.to_string(), rules })
    }
}

struct SynthParser<'a> {
    tokens: &'a [SynthSpanned],
    pos: usize,
}

impl<'a> SynthParser<'a> {
    fn new(tokens: &'a [SynthSpanned]) -> Self {
        SynthParser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&SynthSpanned> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&SynthSpanned> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() { self.pos += 1; }
        tok
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn parse_rules(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules = Vec::new();
        let mut choice_alts: Vec<Alternative> = Vec::new();

        while !self.at_end() {
            let before = self.pos;
            if self.peek().map(|t| &t.token) == Some(&SynthToken::Or) {
                self.advance(); // consume //
                let (items, cardinality) = self.parse_alternative()?;
                choice_alts.push(Alternative { items, cardinality });
            } else {
                // flush pending ordered choice
                if !choice_alts.is_empty() {
                    rules.push(Rule::OrderedChoice(std::mem::take(&mut choice_alts)));
                }
                let items = self.parse_item_sequence()?;
                if !items.is_empty() {
                    rules.push(Rule::Sequential(items));
                }
            }
            if self.pos <= before {
                return Err(format!("synth parser stuck at position {}, token: {:?}",
                    self.pos, self.peek().map(|t| &t.token)));
            }
        }

        if !choice_alts.is_empty() {
            rules.push(Rule::OrderedChoice(choice_alts));
        }

        Ok(rules)
    }

    fn parse_alternative(&mut self) -> Result<(Vec<Item>, Cardinality), String> {
        let cardinality = self.try_cardinality();
        let items = self.parse_item_sequence()?;
        Ok((items, cardinality.unwrap_or(Cardinality::One)))
    }

    fn try_cardinality(&mut self) -> Option<Cardinality> {
        match self.peek().map(|t| &t.token) {
            Some(SynthToken::ZeroOrMore) => { self.advance(); Some(Cardinality::ZeroOrMore) }
            Some(SynthToken::OneOrMore) => { self.advance(); Some(Cardinality::OneOrMore) }
            Some(SynthToken::Optional) => { self.advance(); Some(Cardinality::Optional) }
            _ => None,
        }
    }

    fn parse_item_sequence(&mut self) -> Result<Vec<Item>, String> {
        self.parse_item_sequence_inner(true)
    }

    fn parse_item_sequence_inner(&mut self, stop_at_or: bool) -> Result<Vec<Item>, String> {
        let mut items = Vec::new();

        while !self.at_end() {
            // at rule level, stop at // (next ordered choice alternative)
            if stop_at_or && self.peek().map(|t| &t.token) == Some(&SynthToken::Or) {
                break;
            }
            // stop at closing delimiters
            match self.peek().map(|t| &t.token) {
                Some(SynthToken::RParen) | Some(SynthToken::RBracket) | Some(SynthToken::RBrace)
                | Some(SynthToken::RParenPipe) | Some(SynthToken::RBracketPipe) | Some(SynthToken::RBracePipe) => {
                    break;
                }
                _ => {}
            }

            // inside delimiters, // is inline or — treat as a literal
            if !stop_at_or && self.peek().map(|t| &t.token) == Some(&SynthToken::Or) {
                let spanned = self.advance().unwrap();
                items.push(Item {
                    content: ItemContent::Literal("//".into()),
                    adjacent: spanned.adjacent,
                });
                continue;
            }

            let item = self.parse_item()?;
            items.push(item);
        }

        Ok(items)
    }

    fn parse_item(&mut self) -> Result<Item, String> {
        let spanned = self.peek().ok_or("unexpected end of tokens")?;
        let adjacent = spanned.adjacent;

        // cardinality prefix applies to next item
        if let Some(card) = self.try_cardinality() {
            let inner = self.parse_item()?;
            return Ok(Item {
                content: ItemContent::Repeat {
                    kind: card,
                    inner: Box::new(inner),
                },
                adjacent,
            });
        }

        let spanned = self.advance().ok_or("unexpected end")?;
        let adjacent = spanned.adjacent;

        let content = match &spanned.token {
            SynthToken::Declare(name) => {
                let first = name.chars().next().unwrap_or('a');
                let casing = if first.is_uppercase() { Casing::Pascal } else { Casing::Camel };
                ItemContent::Declare { casing, label: name.clone() }
            }
            SynthToken::DialectRef(name) => {
                ItemContent::DialectRef(name.clone())
            }
            SynthToken::LiteralEscape(content) => {
                ItemContent::Literal(content.clone())
            }
            SynthToken::Ident(name) => {
                // bare token — operator or keyword to match literally
                ItemContent::Literal(name.clone())
            }
            SynthToken::StringLit(_) => {
                ItemContent::LiteralValue
            }
            // opening delimiters — parse content recursively
            SynthToken::LParen => self.parse_delimited(DelimKind::Paren, &SynthToken::RParen)?,
            SynthToken::LBracket => self.parse_delimited(DelimKind::Bracket, &SynthToken::RBracket)?,
            SynthToken::LBrace => self.parse_delimited(DelimKind::Brace, &SynthToken::RBrace)?,
            SynthToken::LParenPipe => self.parse_delimited(DelimKind::ParenPipe, &SynthToken::RParenPipe)?,
            SynthToken::LBracketPipe => self.parse_delimited(DelimKind::BracketPipe, &SynthToken::RBracketPipe)?,
            SynthToken::LBracePipe => self.parse_delimited(DelimKind::BracePipe, &SynthToken::RBracePipe)?,

            other => {
                return Err(format!("unexpected synth token: {:?}", other));
            }
        };

        Ok(Item { content, adjacent })
    }

    fn parse_delimited(&mut self, kind: DelimKind, close: &SynthToken) -> Result<ItemContent, String> {
        let inner = self.parse_item_sequence_inner(false)?;
        if self.peek().map(|t| &t.token) == Some(close) {
            self.advance();
        } else {
            return Err(format!("expected {:?}, got {:?}", close, self.peek().map(|t| &t.token)));
        }
        Ok(ItemContent::Delimited { kind, inner })
    }
}
