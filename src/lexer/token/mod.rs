pub mod kind;
pub mod span;

use self::{kind::TokenKind, span::Span};
use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - <{}, {}>",
            self.kind, self.span.start, self.span.end
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::T;

    #[test]
    fn token_len() {
        let token = Token {
            kind: T![==],
            span: Span { start: 0, end: 2 },
        };
        assert_eq!(token.len(), 2);

        let token = Token {
            kind: T![ident],
            span: Span { start: 2, end: 7 },
        };
        assert_eq!(token.len(), 5);
    }

    #[test]
    fn token_text() {
        let input = "let x = 42;";
        let token = Token {
            kind: T![ident],
            span: Span { start: 4, end: 5 },
        };
        assert_eq!(token.text(input), "x");

        let token = Token {
            kind: T![=],
            span: Span { start: 6, end: 7 },
        };
        assert_eq!(token.text(input), "=");
    }
}
