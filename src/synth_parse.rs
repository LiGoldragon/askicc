/// Synth parser — tokens → aski-core Dialect domain tree.
///
/// All methods on SynthParser struct. Produces aski-core types
/// directly — no intermediate representation.

use aski_core::*;
use crate::synth_token::{SynthToken, SynthSpanned};

pub struct SynthParser<'a> {
    tokens: &'a [SynthSpanned],
    pos: usize,
}

impl<'a> SynthParser<'a> {
    pub fn new(tokens: &'a [SynthSpanned]) -> Self {
        SynthParser { tokens, pos: 0 }
    }

    pub fn parse(mut self, kind: DialectKind) -> Result<Dialect, String> {
        let rules = self.parse_rules()?;
        Ok(Dialect { kind, rules })
    }

    // ── Rule parsing ────────────────────────────────────────

    fn parse_rules(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules = Vec::new();
        let mut choice_alts: Vec<Alternative> = Vec::new();

        while !self.at_end() {
            let before = self.pos;

            if self.peek_token() == Some(&SynthToken::Or) {
                self.advance();
                let alt = self.parse_alternative()?;
                choice_alts.push(alt);
            } else {
                if !choice_alts.is_empty() {
                    rules.push(Rule::OrderedChoice {
                        alternatives: std::mem::take(&mut choice_alts),
                    });
                }
                let items = self.parse_items(true)?;
                if !items.is_empty() {
                    rules.push(Rule::Sequential { items });
                }
            }

            if self.pos <= before {
                return Err(format!("parser stuck at position {}", self.pos));
            }
        }

        if !choice_alts.is_empty() {
            rules.push(Rule::OrderedChoice { alternatives: choice_alts });
        }

        Ok(rules)
    }

    fn parse_alternative(&mut self) -> Result<Alternative, String> {
        let cardinality = self.try_cardinality().unwrap_or(Cardinality::One);
        let items = self.parse_items(true)?;
        Ok(Alternative { items, cardinality })
    }

    // ── Item parsing ────────────────────────────────────────

    fn parse_items(&mut self, stop_at_or: bool) -> Result<Vec<Item>, String> {
        let mut items = Vec::new();

        while !self.at_end() {
            if stop_at_or && self.peek_token() == Some(&SynthToken::Or) {
                break;
            }
            if matches!(self.peek_token(), Some(SynthToken::Close(_))) {
                break;
            }
            if !stop_at_or && self.peek_token() == Some(&SynthToken::Or) {
                let adj = self.peek_adjacent();
                self.advance();
                items.push(Item {
                    content: ItemContent::Literal { token: LiteralToken::InlineOr },
                    adjacent: adj,
                });
                continue;
            }

            items.push(self.parse_item()?);
        }

        Ok(items)
    }

    fn parse_item(&mut self) -> Result<Item, String> {
        let adjacent = self.peek_adjacent();

        if let Some(card) = self.try_cardinality() {
            let inner = self.parse_item()?;
            return Ok(Item {
                content: ItemContent::Repeat { kind: card, inner: Box::new(inner) },
                adjacent,
            });
        }

        let token = self.advance().ok_or("unexpected end of tokens")?.token.clone();

        let content = match token {
            SynthToken::Label(label) => ItemContent::Named { label },
            SynthToken::Keyword(kw) => ItemContent::Keyword { token: kw },
            SynthToken::DialectRef(kind) => ItemContent::DialectRef { target: kind },
            SynthToken::Literal(tok) => ItemContent::Literal { token: tok },
            SynthToken::StringLit => ItemContent::LiteralValue,
            SynthToken::Open(kind) => {
                let inner = self.parse_items(false)?;
                self.expect_close(kind)?;
                ItemContent::Delimited { kind, inner }
            }
            other => return Err(format!("unexpected token in item: {:?}", other)),
        };

        Ok(Item { content, adjacent })
    }

    // ── Token access ────────────────────────────────────────

    fn peek(&self) -> Option<&SynthSpanned> {
        self.tokens.get(self.pos)
    }

    fn peek_token(&self) -> Option<&SynthToken> {
        self.peek().map(|s| &s.token)
    }

    fn peek_adjacent(&self) -> bool {
        self.peek().map(|s| s.adjacent).unwrap_or(false)
    }

    fn advance(&mut self) -> Option<&SynthSpanned> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() { self.pos += 1; }
        tok
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn expect_close(&mut self, kind: DelimKind) -> Result<(), String> {
        match self.peek_token() {
            Some(SynthToken::Close(k)) if *k == kind => { self.advance(); Ok(()) }
            other => Err(format!("expected close {:?}, got {:?}", kind, other)),
        }
    }

    fn try_cardinality(&mut self) -> Option<Cardinality> {
        match self.peek_token() {
            Some(SynthToken::ZeroOrMore) => { self.advance(); Some(Cardinality::ZeroOrMore) }
            Some(SynthToken::OneOrMore) => { self.advance(); Some(Cardinality::OneOrMore) }
            Some(SynthToken::Optional) => { self.advance(); Some(Cardinality::Optional) }
            _ => None,
        }
    }
}
