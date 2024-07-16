mod ast;
mod expressions;
mod token_iter;

use crate::{
    lexer::{Token, TokenKind},
    T,
};
use std::iter::Peekable;
use token_iter::TokenIter;

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Parser<'input, TokenIter<'input>> {
        Self {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    /// Get the source text of a token.
    pub fn text(&self, token: Token) -> &'input str {
        token.text(&self.input)
    }

    /// Look-ahead one token and see what kind of token it is.
    pub(crate) fn peek(&mut self) -> TokenKind {
        self.tokens.peek().map(|t| t.kind).unwrap_or(T![EOF])
    }

    /// Check if the next token is some `kind` of token.\
    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    /// Get the next token.
    pub(crate) fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Move forward one token in the input and check
    /// that we pass the kind of token we expect.
    pub(crate) fn consume(&mut self, expected: TokenKind) {
        let token = self.next().expect(&format!(
            "Expected to consume `{}`, but there was no next token",
            expected,
        ));

        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token.kind
        );
    }
}
