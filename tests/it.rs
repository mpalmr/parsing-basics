use parsing_basics::{lexer::*, T};

/// Walks `$tokens` and compares them to the given kinds.
macro_rules! assert_tokens {
    ($tokens:ident, [$($Kind:expr,)*]) => {
        {
            let mut it = $tokens.iter();
            $(
                let token = it.next().expect("not enough tokens");
                assert_eq!(token.kind, $Kind);
            )*
        }
    };
}

#[test]
fn single_char_tokens() {
    let mut lexer = Lexer::new("+-(.):");
    let tokens = lexer.tokenize();
    assert_tokens!(tokens, [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]);
}

#[test]
fn unknown_input() {
    let mut lexer = Lexer::new("{$$$$$$$+");
    let tokens = lexer.tokenize();
    assert_tokens!(tokens, [T!['{'], T![error], T![+], T![EOF],]);
}

#[test]
fn token_spans() {
    {
        let mut lexer = Lexer::new("+-(.):");
        let tokens = lexer.tokenize();

        let dot = tokens[3];
        assert_eq!(dot.kind, T![.]);
        assert_eq!(dot.span, (3..4).into());
    }
    {
        let mut lexer = Lexer::new("{$$$$$$$+");
        let tokens = lexer.tokenize();

        let error = tokens[1];
        assert_eq!(error.kind, T![error]);
        assert_eq!(error.span, (1..8).into());
    }
}
