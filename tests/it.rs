use parsing_basics::{lexer::*, parser::{ast, Parser}, T};
use unindent::unindent;

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
    assert_tokens!(
        tokens,
        [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]
    );
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

#[test]
fn single_char_tokens_with_whitespace() {
    let mut lexer = Lexer::new("   + -  (.): ");
    let tokens = lexer.tokenize();

    let leading_space = &tokens[0];
    assert_eq!(leading_space.kind, T![ws]);
    assert_eq!(leading_space.len(), 3);

    let space_after_minus = &tokens[4];
    assert_eq!(space_after_minus.kind, T![ws]);
    assert_eq!(space_after_minus.len(), 2);

    let trailing_space = &tokens[9];
    assert_eq!(trailing_space.kind, T![ws]);
    assert_eq!(trailing_space.len(), 1);

    let tokens: Vec<_> = tokens.into_iter().filter(|t| t.kind != T![ws]).collect();
    assert_tokens!(
        tokens,
        [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]
    );
}

#[test]
fn maybe_multiple_char_tokens() {
    let mut lexer = Lexer::new("&&=<=_!=||");
    let tokens = lexer.tokenize();
    assert_tokens!(
        tokens,
        [T![&&], T![=], T![<=], T![_], T![!=], T![||], T![EOF],]
    );
}

#[test]
fn keywords() {
    let mut lexer = Lexer::new("if let = struct else fn");
    let tokens: Vec<_> = lexer
        .tokenize()
        .into_iter()
        .filter(|token| token.kind != T![ws])
        .collect();

    assert_tokens!(
        tokens,
        [
            T![if],
            T![let],
            T![=],
            T![struct],
            T![else],
            T![fn],
            T![EOF],
        ]
    );
}

#[test]
#[rustfmt::skip]
fn function() {
    let input = unindent(r#"
        // tests stuff
        fn test(var: Type, var2_: bool) {
            let x = "String content \" test" + 7 / 27.3e-2^4;
            let chars = x.chars();
            if let Some(c) = chars.next() {
                x = x + c;
            } else if !var2_ {
                x = x + ",";
            }
        }
    "#);

    let mut lexer = Lexer::new(input.as_str());
    let tokens: Vec<_> = lexer.tokenize().into_iter().filter(|t| t.kind != T![ws]).collect();

    assert_tokens!(tokens, [
        // `// tests stuff`
        T![comment],

        // fn test(var: Type, var2_: bool) {
        T![fn], T![ident], T!['('],
            T![ident], T![:], T![ident], T![,],
            T![ident], T![:], T![ident],
        T![')'], T!['{'],

            // let x = "String content \" test" + 7 / 27.3e-2^4;
            T![let], T![ident], T![=],
                T![string], T![+], T![int], T![/], T![float], T![^], T![int], T![;],

            // let chars = x.chars();
            T![let], T![ident], T![=],
                T![ident], T![.], T![ident], T!['('], T![')'], T![;],

            // if let Some(c) = chars.next() {
            T![if], T![let], T![ident], T!['('], T![ident], T![')'], T![=],
                T![ident], T![.], T![ident], T!['('], T![')'],
            T!['{'],

                // x = x + c;
                T![ident], T![=], T![ident], T![+], T![ident], T![;],

            // } else if !var2_ {
            T!['}'], T![else], T![if], T![!], T![ident], T!['{'],

                // x = x + ",";
                T![ident], T![=], T![ident], T![+], T![string], T![;],

            T!['}'],
        T!['}'],
        T![EOF],
    ]);
}

#[test]
#[rustfmt::skip]
fn struct_def() {
    let input = unindent(r#"
        struct Foo<T> {
            bar: Bar<T>,
        }
    "#);
    let input = input.as_str();

    let mut lexer = Lexer::new(input);
    let tokens: Vec<_> = lexer
        .tokenize()
        .into_iter()
        .filter(|token| token.kind != T![ws])
        .collect();

    assert_tokens!(tokens, [
        // struct Foo<T> {
        T![struct], T![ident], T![<], T![ident], T![>], T!['{'],

            // bar: Bar<T>,
            T![ident], T![:], T![ident], T![<], T![ident], T![>], T![,],

        T!['}'],
    ]);

    let foo = tokens[1];
    assert_eq!(foo.text(input), "Foo");

    let bar = tokens[6];
    assert_eq!(bar.span, (20..23).into());
    assert_eq!(bar.text(input), "bar");
}

#[test]
fn parse_expression() {
    fn parse(input: &str) -> ast::Expr {
        let mut parser = Parser::new(input);
        parser.parse_expression(0)
    }

    // Weird spaces are to test that whitespace gets filtered out
    assert_eq!(parse("42"), ast::Expr::Literal(ast::Lit::Int(42)));
    assert_eq!(parse("  2.7768"), ast::Expr::Literal(ast::Lit::Float(2.7768)));
    assert_eq!(
        parse(r#""I am a String!""#),
        ast::Expr::Literal(ast::Lit::Str("I am a String!".to_string())),
    );
    assert_eq!(parse("foo"), ast::Expr::Ident("foo".to_string()));

    assert_eq!(
        parse("bar (  x, 2)"),
        ast::Expr::FnCall {
            fn_name: "bar".to_string(),
            args: vec![
                ast::Expr::Ident("x".to_string()),
                ast::Expr::Literal(ast::Lit::Int(2)),
            ],
        },
    );

    assert_eq!(
        parse("!  is_visible"),
        ast::Expr::PrefixOp {
            op: T![!],
            expr: Box::new(ast::Expr::Ident("is_visible".to_string())),
        }
    );

    assert_eq!(
        parse("(-13)"),
        ast::Expr::PrefixOp {
            op: T![-],
            expr: Box::new(ast::Expr::Literal(ast::Lit::Int(13))),
        }
    );
}

#[test]
fn parse_binary_expressions() {
    fn parse(input: &str) -> ast::Expr {
        let mut parser = Parser::new(input);
        parser.parse_expression(0)
    }

    assert_eq!(parse("4 + 2 * 3").to_string(), "(4 + (2 * 3))");
    assert_eq!(parse("4 * 2 + 3").to_string(), "((4 * 2) + 3)");
    assert_eq!(parse("4 - 2 - 3").to_string(), "((4 - 2) - 3)");
    assert_eq!(parse("4 ^ 2 ^ 3").to_string(), "(4 ^ (2 ^ 3))");
}
