use super::TokenKind;
use crate::T;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) struct Rule {
    pub kind: TokenKind,
    pub matches: fn(&str) -> Option<u32>,
}

pub(crate) fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            kind: T![!],
            matches: |input| match_single_char(input, '!'),
        },
        Rule {
            kind: T![=],
            matches: |input| match_single_char(input, '='),
        },
        Rule {
            kind: T![/],
            matches: |input| match_single_char(input, '/'),
        },
        Rule {
            kind: T![_],
            matches: |input| match_single_char(input, '_'),
        },
        Rule {
            kind: T![<],
            matches: |input| match_single_char(input, '<'),
        },
        Rule {
            kind: T![>],
            matches: |input| match_single_char(input, '>'),
        },
        Rule {
            kind: T![==],
            matches: |input| match_two_chars(input, '=', '='),
        },
        Rule {
            kind: T![!=],
            matches: |input| match_two_chars(input, '!', '='),
        },
        Rule {
            kind: T![&&],
            matches: |input| match_two_chars(input, '&', '&'),
        },
        Rule {
            kind: T![||],
            matches: |input| match_two_chars(input, '|', '|'),
        },
        Rule {
            kind: T![<=],
            matches: |input| match_two_chars(input, '<', '='),
        },
        Rule {
            kind: T![>=],
            matches: |input| match_two_chars(input, '>', '='),
        },
        Rule {
            kind: T![let],
            matches: |input| match_keyword(input, "let"),
        },
        Rule {
            kind: T![fn],
            matches: |input| match_keyword(input, "fn"),
        },
        Rule {
            kind: T![struct],
            matches: |input| match_keyword(input, "struct"),
        },
        Rule {
            kind: T![if],
            matches: |input| match_keyword(input, "if"),
        },
        Rule {
            kind: T![else],
            matches: |input| match_keyword(input, "else"),
        },
        Rule {
            kind: T![string],
            matches: move |input| match_regex(input, &STRING_REGEX),
        },
        Rule {
            kind: T![comment],
            matches: move |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind: T![int],
            matches: |input| {
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map(|(pos, _)| pos as u32 + 1)
            },
        },
        Rule {
            kind: T![float],
            matches: |input| match_regex(input, &FLOAT_REGEX),
        },
        Rule {
            kind: T![ident],
            matches: |input| match_regex(input, &IDENTIFIER_REGEX),
        },
    ]
}

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

fn match_single_char(input: &str, c: char) -> Option<u32> {
    input
        .chars()
        .next()
        .and_then(|ch| if ch == c { Some(1) } else { None })
}

fn match_two_chars(input: &str, first: char, second: char) -> Option<u32> {
    if input.len() >= 2 {
        match_single_char(input, first)
            .and_then(|_| match_single_char(&input[1..], second).map(|_| 2))
    } else {
        None
    }
}

fn match_keyword(input: &str, keyword: &str) -> Option<u32> {
    input.starts_with(keyword).then(|| keyword.len() as u32)
}

fn match_regex(input: &str, r: &Regex) -> Option<u32> {
    r.find(input).map(|regex_match| regex_match.end() as u32)
}

lazy_static! {
    static ref STRING_REGEX: Regex = Regex::new(r#"^"((\\"|\\\\)|[^\\"])*""#).unwrap();
    static ref COMMENT_REGEX: Regex = Regex::new(r#"^//[^\n]*\n"#).unwrap();
    static ref FLOAT_REGEX: Regex =
        Regex::new(r#"^((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#).unwrap();
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r##"^([A-Za-z]|_)([A-Za-z]|_|\d)*"##).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unambiguous_single_char() {
        use super::*;
        use crate::T;

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
