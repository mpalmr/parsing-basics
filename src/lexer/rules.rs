use crate::T;
use super::token::kind::TokenKind;

/// If the given character is a character that _only_
/// represents a token of length 1,
/// this method returns the corresponding `TokenKind`.
/// Note that this method will return `None` for characters
/// like `=` that may also occur at the first position
/// of longer tokens (here `==`).
pub(crate) const fn unambiguous_single_char(c: char) -> Option<TokenKind> {
    Some(match c {
        '+' => T![+],
        '-' => T![-],
        '*' => T![*],
        '^' => T![^],
        '.' => T![.],
        ',' => T![,],
        '[' => T!['['],
        ']' => T![']'],
        '{' => T!['{'],
        '}' => T!['}'],
        '(' => T!['('],
        ')' => T![')'],
        ':' => T![:],
        ';' => T![;],
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unambiguous_single_char() {
        use crate::T;
        use super::*;

        assert_eq!(unambiguous_single_char('+'), Some(T![+]));
        assert_eq!(unambiguous_single_char('-'), Some(T![-]));
        assert_eq!(unambiguous_single_char('*'), Some(T![*]));
        assert_eq!(unambiguous_single_char('^'), Some(T![^]));
        assert_eq!(unambiguous_single_char('.'), Some(T![.]));
        assert_eq!(unambiguous_single_char(','), Some(T![,]));
        assert_eq!(unambiguous_single_char('['), Some(T!['[']));
        assert_eq!(unambiguous_single_char(']'), Some(T![']']));
        assert_eq!(unambiguous_single_char('{'), Some(T!['{']));
        assert_eq!(unambiguous_single_char('}'), Some(T!['}']));
        assert_eq!(unambiguous_single_char('('), Some(T!['(']));
        assert_eq!(unambiguous_single_char(')'), Some(T![')']));
        assert_eq!(unambiguous_single_char(':'), Some(T![:]));
        assert_eq!(unambiguous_single_char(';'), Some(T![;]));

        assert_eq!(unambiguous_single_char('a'), None);
        assert_eq!(unambiguous_single_char('b'), None);
        assert_eq!(unambiguous_single_char('c'), None);
        assert_eq!(unambiguous_single_char('d'), None);
        assert_eq!(unambiguous_single_char('e'), None);
        assert_eq!(unambiguous_single_char('f'), None);
        assert_eq!(unambiguous_single_char('g'), None);
        assert_eq!(unambiguous_single_char('h'), None);
        assert_eq!(unambiguous_single_char('i'), None);
        assert_eq!(unambiguous_single_char('j'), None);
        assert_eq!(unambiguous_single_char('k'), None);
        assert_eq!(unambiguous_single_char('l'), None);
        assert_eq!(unambiguous_single_char('m'), None);
        assert_eq!(unambiguous_single_char('n'), None);
        assert_eq!(unambiguous_single_char('o'), None);
        assert_eq!(unambiguous_single_char('p'), None);
        assert_eq!(unambiguous_single_char('q'), None);
        assert_eq!(unambiguous_single_char('r'), None);
        assert_eq!(unambiguous_single_char('s'), None);
        assert_eq!(unambiguous_single_char('t'), None);
    }
}
