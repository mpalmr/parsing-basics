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
        $crate::lexer::token::TokenKind::Plus
    };
    [-] => {
        $crate::lexer::token::TokenKind::Minus
    };
    [*] => {
        $crate::lexer::token::TokenKind::Times
    };
    [/] => {
        $crate::lexer::token::TokenKind::Slash
    };
    [^] => {
        $crate::lexer::token::TokenKind::Pow
    };
    [=] => {
        $crate::lexer::token::TokenKind::Eq
    };
    [.] => {
        $crate::lexer::token::TokenKind::Dot
    };
    [,] => {
        $crate::lexer::token::TokenKind::Comma
    };
    [_] => {
        $crate::lexer::token::TokenKind::Underscore
    };
    [!] => {
        $crate::lexer::token::TokenKind::Bang
    };
    [&] => {
        $crate::lexer::token::TokenKind::Ampersand
    };
    [|] => {
        $crate::lexer::token::TokenKind::Bar
    };
    [:] => {
        $crate::lexer::token::TokenKind::Colon
    };
    [;] => {
        $crate::lexer::token::TokenKind::SemiColon
    };
    [<] => {
        $crate::lexer::token::TokenKind::LAngle
    };
    [>] => {
        $crate::lexer::token::TokenKind::RAngle
    };
    ['['] => {
        $crate::lexer::token::TokenKind::LSquare
    };
    [']'] => {
        $crate::lexer::token::TokenKind::RSquare
    };
    ['{'] => {
        $crate::lexer::token::TokenKind::LBrace
    };
    ['}'] => {
        $crate::lexer::token::TokenKind::RBrace
    };
    ['('] => {
        $crate::lexer::token::TokenKind::LParen
    };
    [')'] => {
        $crate::lexer::token::TokenKind::RParen
    };
    [string] => {
        $crate::lexer::token::TokenKind::String
    };
    [comment] => {
        $crate::lexer::token::TokenKind::Comment
    };
    [int] => {
        $crate::lexer::token::TokenKind::Int
    };
    [float] => {
        $crate::lexer::token::TokenKind::Float
    };
    [ident] => {
        $crate::lexer::token::TokenKind::Identifier
    };
    [let] => {
        $crate::lexer::token::TokenKind::KeywordLet
    };
    [fn] => {
        $crate::lexer::token::TokenKind::KeywordFn
    };
    [struct] => {
        $crate::lexer::token::TokenKind::KeywordStruct
    };
    [if] => {
        $crate::lexer::token::TokenKind::KeywordIf
    };
    [else] => {
        $crate::lexer::token::TokenKind::KeywordElse
    };
    [&&] => {
        $crate::lexer::token::TokenKind::And
    };
    [||] => {
        $crate::lexer::token::TokenKind::Or
    };
    [==] => {
        $crate::lexer::token::TokenKind::Eqq
    };
    [!=] => {
        $crate::lexer::token::TokenKind::Neq
    };
    [>=] => {
        $crate::lexer::token::TokenKind::Geq
    };
    [<=] => {
        $crate::lexer::token::TokenKind::Leq
    };
    [error] => {
        $crate::lexer::token::TokenKind::Error
    };
    [ws] => {
        $crate::lexer::token::TokenKind::Whitespace
    };
    [EOF] => {
        $crate::lexer::token::TokenKind::Eof
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
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
        })
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
