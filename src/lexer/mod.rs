mod rules;
mod token;

use self::{rules::unambiguous_single_char, token::{span::Span, Token}};

pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self
    }

    /// Returns `None` if the lexer cannot find a token at the start of `input`.
    fn valid_token(&self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if let Some(kind) = unambiguous_single_char(next) {
            (1, kind)
        } else {
            return None;
        };

        Some(Token {
            kind,
            span: Span {
                start: 0,
                end: len,
            },
        })
    }
}
