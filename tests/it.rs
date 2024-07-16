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
    let tokens = Lexer::new().tokenize("+-(.):");
    assert_tokens!(tokens, [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]);
}

#[test]
fn unknown_input() {
    let tokens = Lexer::new().tokenize("{$$$$$$$+");
    assert_tokens!(tokens, [T!['{'], T![error], T![+], T![EOF],]);
}
