/// Synth lexer — space-preserving tokenizer for .synth files.
///
/// Unlike the aski lexer (which ignores whitespace), the synth lexer
/// tracks whether tokens are adjacent or separated by space. This is
/// because space in synth rules is significant — it determines whether
/// aski source tokens must be adjacent.

#[derive(Debug, Clone, PartialEq)]
pub enum SynthToken {
    /// ;; to end of line
    Comment,
    /// //
    Or,
    /// +
    OneOrMore,
    /// *
    ZeroOrMore,
    /// ?
    Optional,
    /// <name>
    DialectRef(String),
    /// @Name or @name
    Declare(String),
    /// _content_ — literal escape
    LiteralEscape(String),
    /// PascalCase or camelCase identifier (bare, not after @)
    Ident(String),
    /// "string"
    StringLit(String),
    /// ( ) [ ] { }
    LParen, RParen,
    LBracket, RBracket,
    LBrace, RBrace,
    /// (| |) [| |] {| |}
    LParenPipe, RParenPipe,
    LBracketPipe, RBracketPipe,
    LBracePipe, RBracePipe,
}

#[derive(Debug, Clone)]
pub struct SynthSpanned {
    pub token: SynthToken,
    pub adjacent: bool,
    pub start: usize,
    pub end: usize,
}

pub fn synth_lex(source: &str) -> Result<Vec<SynthSpanned>, String> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut pos = 0;
    let mut last_end: usize = 0;

    while pos < bytes.len() {
        let b = bytes[pos];

        // newlines and spaces are significant for adjacency
        match b {
            b' ' | b'\t' | b'\n' | b'\r' => {
                pos += 1;
                continue;
            }
            _ => {}
        }

        let adjacent = pos == last_end && !tokens.is_empty();
        let start = pos;

        let token = match b {
            // comment ;; to end of line
            b';' if pos + 1 < bytes.len() && bytes[pos + 1] == b';' => {
                while pos < bytes.len() && bytes[pos] != b'\n' {
                    pos += 1;
                }
                last_end = pos;
                continue; // skip comments
            }

            // // — or operator
            b'/' if pos + 1 < bytes.len() && bytes[pos + 1] == b'/' => {
                pos += 2;
                SynthToken::Or
            }

            // <= and >= operators (must check before < dialect ref)
            b'<' if pos + 1 < bytes.len() && bytes[pos + 1] == b'=' => {
                pos += 2;
                SynthToken::Ident("<=".into())
            }
            b'>' if pos + 1 < bytes.len() && bytes[pos + 1] == b'=' => {
                pos += 2;
                SynthToken::Ident(">=".into())
            }

            // dialect ref <Name> — only if followed by an identifier char
            b'<' if pos + 1 < bytes.len() && bytes[pos + 1].is_ascii_alphabetic() => {
                pos += 1;
                let name_start = pos;
                while pos < bytes.len() && bytes[pos] != b'>' {
                    pos += 1;
                }
                let name = String::from_utf8_lossy(&bytes[name_start..pos]).to_string();
                if pos < bytes.len() { pos += 1; } // skip >
                SynthToken::DialectRef(name)
            }

            // literal escape _content_
            b'_' => {
                pos += 1;
                let content_start = pos;
                while pos < bytes.len() && bytes[pos] != b'_' {
                    pos += 1;
                }
                let content = String::from_utf8_lossy(&bytes[content_start..pos]).to_string();
                if pos < bytes.len() { pos += 1; } // skip closing _
                SynthToken::LiteralEscape(content)
            }

            // @ declare
            b'@' => {
                pos += 1;
                let name_start = pos;
                while pos < bytes.len() && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                    pos += 1;
                }
                let name = String::from_utf8_lossy(&bytes[name_start..pos]).to_string();
                SynthToken::Declare(name)
            }

            // cardinality
            b'+' => { pos += 1; SynthToken::OneOrMore }
            b'*' => { pos += 1; SynthToken::ZeroOrMore }
            b'?' => { pos += 1; SynthToken::Optional }

            // piped delimiters (must check before single-char)
            b'(' if pos + 1 < bytes.len() && bytes[pos + 1] == b'|' => {
                pos += 2; SynthToken::LParenPipe
            }
            b'[' if pos + 1 < bytes.len() && bytes[pos + 1] == b'|' => {
                pos += 2; SynthToken::LBracketPipe
            }
            b'{' if pos + 1 < bytes.len() && bytes[pos + 1] == b'|' => {
                pos += 2; SynthToken::LBracePipe
            }
            b'|' if pos + 1 < bytes.len() && bytes[pos + 1] == b')' => {
                pos += 2; SynthToken::RParenPipe
            }
            b'|' if pos + 1 < bytes.len() && bytes[pos + 1] == b']' => {
                pos += 2; SynthToken::RBracketPipe
            }
            b'|' if pos + 1 < bytes.len() && bytes[pos + 1] == b'}' => {
                pos += 2; SynthToken::RBracePipe
            }

            // simple delimiters
            b'(' => { pos += 1; SynthToken::LParen }
            b')' => { pos += 1; SynthToken::RParen }
            b'[' => { pos += 1; SynthToken::LBracket }
            b']' => { pos += 1; SynthToken::RBracket }
            b'{' => { pos += 1; SynthToken::LBrace }
            b'}' => { pos += 1; SynthToken::RBrace }

            // string literal
            b'"' => {
                pos += 1;
                let str_start = pos;
                while pos < bytes.len() && bytes[pos] != b'"' {
                    if bytes[pos] == b'\\' { pos += 1; } // skip escaped char
                    pos += 1;
                }
                let content = String::from_utf8_lossy(&bytes[str_start..pos]).to_string();
                if pos < bytes.len() { pos += 1; } // skip closing "
                SynthToken::StringLit(content)
            }

            // bare identifier (not after @)
            b'A'..=b'Z' | b'a'..=b'z' => {
                let name_start = pos;
                while pos < bytes.len() && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                    pos += 1;
                }
                let name = String::from_utf8_lossy(&bytes[name_start..pos]).to_string();
                SynthToken::Ident(name)
            }

            // bare < and > as operators (not dialect refs)
            b'<' => { pos += 1; SynthToken::Ident("<".into()) }
            b'>' => { pos += 1; SynthToken::Ident(">".into()) }

            // bare operators (not synth-conflicting)
            _ if b.is_ascii_graphic() => {
                let op_start = pos;
                while pos < bytes.len()
                    && bytes[pos].is_ascii_graphic()
                    && !b" \t\n\r()[]{}@<>_\"".contains(&bytes[pos])
                    && !(bytes[pos] == b';' && pos + 1 < bytes.len() && bytes[pos + 1] == b';')
                    && !(bytes[pos] == b'/' && pos + 1 < bytes.len() && bytes[pos + 1] == b'/')
                {
                    pos += 1;
                }
                let op = String::from_utf8_lossy(&bytes[op_start..pos]).to_string();
                if op.is_empty() {
                    return Err(format!("empty operator at position {}", pos));
                }
                SynthToken::Ident(op)
            }

            _ => {
                return Err(format!("unexpected byte '{}' at position {}", b as char, pos));
            }
        };

        last_end = pos;
        tokens.push(SynthSpanned {
            token,
            adjacent,
            start,
            end: pos,
        });
    }

    Ok(tokens)
}
