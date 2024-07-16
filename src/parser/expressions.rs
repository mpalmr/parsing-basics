use crate::{lexer::Token, T};
use super::{ast, Parser};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_expression(&mut self) -> ast::Expr {
        match self.peek() {
            lit @ T![int] | lit @ T![float] | lit @ T![string] => {
                let literal_text = {
                    // the calls on `self` need to be split, because `next` takes
                    // `&mut self` if `peek` is not `T![EOF]`, then there must be
                    // a next token
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };

                let lit = match lit {
                    T![int] => ast::Lit::Int(
                        literal_text
                            .parse()
                            .expect(&format!("invalid integer literal: `{}`", literal_text))
                    ),

                    T![float] => ast::Lit::Float(
                        literal_text
                            .parse()
                            .expect(&format!("invalid floating point literal: `{}`", literal_text))
                    ),

                    T![string] => ast::Lit::Str(
                        // trim the quotation marks
                        literal_text[1..(literal_text.len() - 1)].to_string()
                    ),
                    _ => unreachable!(),
                };

                ast::Expr::Literal(lit)
            },

            _ => todo!(),
        }
    }
}
