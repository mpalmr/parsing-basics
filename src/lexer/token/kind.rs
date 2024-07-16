use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    // Single characters
    Plus,
    Minus,
    Times,
    Slash,
    Pow,
    Eq,
    Dot,
    Comma,
    Underscore,
    Bang,
    Ampersand,
    Bar,
    Colon,
    SemiColon,
    // Brackets
    LAngle,
    RAngle,
    LSquare,
    RSquare,
    LBrace,
    RBrace,
    LParen,
    RParen,
    // Multiple characters
    String,
    Comment,
    Int,
    Float,
    Identifier,
    KeywordLet,
    KeywordFn,
    KeywordStruct,
    KeywordIf,
    KeywordElse,
    // Operators
    And,
    Or,
    Eqq,
    Neq,
    Geq,
    Leq,
    // Misc,
    Error,
    Whitespace,
    Eof,
}

#[macro_export]
macro_rules! T {
    [+] => {
        $crate::lexer::token::kind::TokenKind::Plus
    };
    [-] => {
        $crate::lexer::token::kind::TokenKind::Minus
    };
    [*] => {
        $crate::lexer::token::kind::TokenKind::Times
    };
    [/] => {
        $crate::lexer::token::kind::TokenKind::Slash
    };
    [^] => {
        $crate::lexer::token::kind::TokenKind::Pow
    };
    [=] => {
        $crate::lexer::token::kind::TokenKind::Eq
    };
    [.] => {
        $crate::lexer::token::kind::TokenKind::Dot
    };
    [,] => {
        $crate::lexer::token::kind::TokenKind::Comma
    };
    [_] => {
        $crate::lexer::token::kind::TokenKind::Underscore
    };
    [!] => {
        $crate::lexer::token::kind::TokenKind::Bang
    };
    [&] => {
        $crate::lexer::token::kind::TokenKind::Ampersand
    };
    [|] => {
        $crate::lexer::token::kind::TokenKind::Bar
    };
    [:] => {
        $crate::lexer::token::kind::TokenKind::Colon
    };
    [;] => {
        $crate::lexer::token::kind::TokenKind::SemiColon
    };
    [<] => {
        $crate::lexer::token::kind::TokenKind::LAngle
    };
    [>] => {
        $crate::lexer::token::kind::TokenKind::RAngle
    };
    ['['] => {
        $crate::lexer::token::kind::TokenKind::LSquare
    };
    [']'] => {
        $crate::lexer::token::kind::TokenKind::RSquare
    };
    ['{'] => {
        $crate::lexer::token::kind::TokenKind::LBrace
    };
    ['}'] => {
        $crate::lexer::token::kind::TokenKind::RBrace
    };
    ['('] => {
        $crate::lexer::token::kind::TokenKind::LParen
    };
    [')'] => {
        $crate::lexer::token::kind::TokenKind::RParen
    };
    [string] => {
        $crate::lexer::token::kind::TokenKind::String
    };
    [comment] => {
        $crate::lexer::token::kind::TokenKind::Comment
    };
    [int] => {
        $crate::lexer::token::kind::TokenKind::Int
    };
    [float] => {
        $crate::lexer::token::kind::TokenKind::Float
    };
    [ident] => {
        $crate::lexer::token::kind::TokenKind::Identifier
    };
    [let] => {
        $crate::lexer::token::kind::TokenKind::KeywordLet
    };
    [fn] => {
        $crate::lexer::token::kind::TokenKind::KeywordFn
    };
    [struct] => {
        $crate::lexer::token::kind::TokenKind::KeywordStruct
    };
    [if] => {
        $crate::lexer::token::kind::TokenKind::KeywordIf
    };
    [else] => {
        $crate::lexer::token::kind::TokenKind::KeywordElse
    };
    [&&] => {
        $crate::lexer::token::kind::TokenKind::And
    };
    [||] => {
        $crate::lexer::token::kind::TokenKind::Or
    };
    [==] => {
        $crate::lexer::token::kind::TokenKind::Eqq
    };
    [!=] => {
        $crate::lexer::token::kind::TokenKind::Neq
    };
    [>=] => {
        $crate::lexer::token::kind::TokenKind::Geq
    };
    [<=] => {
        $crate::lexer::token::kind::TokenKind::Leq
    };
    [error] => {
        $crate::lexer::token::kind::TokenKind::Error
    };
    [ws] => {
        $crate::lexer::token::kind::TokenKind::Whitespace
    };
    [EOF] => {
        $crate::lexer::token::kind::TokenKind::Eof
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Single characters
                T![+] => "+",
                T![-] => "-",
                T![*] => "*",
                T![/] => "/",
                T![^] => "^",
                T![=] => "=",
                T![.] => ".",
                T![,] => ",",
                T![_] => "_",
                T![!] => "!",
                T![&] => "&",
                T![|] => "|",
                T![:] => ":",
                T![;] => ";",
                // Brackets
                T![<] => "<",
                T![>] => ">",
                T!['['] => "[",
                T![']'] => "]",
                T!['{'] => "{",
                T!['}'] => "}",
                T!['('] => "(",
                T![')'] => ")",
                // Multiple characters
                T![string] => "String",
                T![comment] => "// Comment",
                T![int] => "Int",
                T![float] => "Float",
                T![ident] => "Identifier",
                T![let] => "let",
                T![fn] => "fn",
                T![struct] => "struct",
                T![if] => "if",
                T![else] => "else",
                // Operators
                T![&&] => "&&",
                T![||] => "||",
                T![==] => "==",
                T![!=] => "!=",
                T![>=] => ">=",
                T![<=] => "<=",
                // Misc
                T![error] => "<?>",
                T![ws] => "<WS>",
                T![EOF] => "<EOF>",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_kind_display() {
        assert_eq!(T![+].to_string(), "+");
        assert_eq!(T![<=].to_string(), "<=");
        assert_eq!(T![let].to_string(), "let");
        assert_eq!(T![comment].to_string(), "// Comment");
        assert_eq!(T![error].to_string(), "<?>");
        assert_eq!(T![ws].to_string(), "<WS>");
        assert_eq!(T![EOF].to_string(), "<EOF>");
    }
}
