use super::{ast, Parser};
use crate::{lexer::{Token, TokenKind}, T};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_expression(&mut self, binding_power: u8) -> ast::Expr {
        let mut lhs = match self.peek() {
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
                            .expect(&format!("invalid integer literal: `{}`", literal_text)),
                    ),

                    T![float] => ast::Lit::Float(literal_text.parse().expect(&format!(
                        "invalid floating point literal: `{}`",
                        literal_text
                    ))),

                    T![string] => ast::Lit::Str(
                        // trim the quotation marks
                        literal_text[1..(literal_text.len() - 1)].to_string(),
                    ),
                    _ => unreachable!(),
                };

                ast::Expr::Literal(lit)
            }

            T![ident] => {
                let name = {
                    let ident_token = self.next().unwrap();
                    self.text(ident_token).to_string()
                };

                if !self.at(T!['(']) {
                    // plain identifier
                    ast::Expr::Ident(name)
                } else {
                    // function call
                    let mut args = Vec::new();
                    self.consume(T!['(']);

                    while !self.at(T![')']) {
                        args.push(self.parse_expression(0));
                        if self.at(T![,]) {
                            self.consume(T![,]);
                        }
                    }

                    self.consume(T![')']);
                    ast::Expr::FnCall {
                        fn_name: name,
                        args,
                    }
                }
            }

            T!['('] => {
                // Ther is no AST node for grouped expressions.
                // Parentheses just influence the tree structure.
                self.consume(T!['(']);
                let expr = self.parse_expression(0);
                self.consume(T![')']);
                expr
            }

            op @ T![+] | op @ T![-] | op @ T![!] => {
                self.consume(op);
                let ((), right_binding_power) = op.prefix_binding_power();
                ast::Expr::PrefixOp {
                    op,
                    expr: Box::new(self.parse_expression(right_binding_power)),
                }
            }

            kind => {
                panic!("Unknown start of expression: `{}`", kind);
            }
        };

        loop {
            let op = match self.peek() {
                op @ T![+]
                | op @ T![-]
                | op @ T![*]
                | op @ T![/]
                | op @ T![^]
                | op @ T![==]
                | op @ T![!=]
                | op @ T![&&]
                | op @ T![||]
                | op @ T![<]
                | op @ T![<=]
                | op @ T![>]
                | op @ T![>=]
                | op @ T![!] => op,
                T![EOF] => break,
                T![')'] | T!['}'] | T![,] | T![;] => break,
                kind => panic!("Unknown operator: `{}`", kind),
            };

            if let Some((left_binding_power, right_binding_power)) = op.infix_binding_power() {
                if left_binding_power < binding_power {
                    break;
                }

                self.consume(op);
                let rhs = self.parse_expression(right_binding_power);
                lhs = ast::Expr::InfixOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };

                continue;
            }
            break;
        }

        lhs
    }
}

trait Operator {
    /// Prefix operators bind their operands to the right.
    fn prefix_binding_power(&self) -> ((), u8);

    /// Infix operators bind two operands, lhs and rhs.
    fn infix_binding_power(&self) -> Option<(u8, u8)>;

    /// Postfix operators bind their operand to the left.
    fn postfix_binding_power(&self) -> Option<(u8, ())>;
}

impl Operator for TokenKind {
    fn prefix_binding_power(&self) -> ((), u8) {
        match self {
            T![+] | T![-] | T![!] => ((), 51),
            // Prefixes are the only operators we have already seen
            // when we call this, so we know the token must be
            // one of the above.
            _ => unreachable!(),
        }
    }

    fn infix_binding_power(&self) -> Option<(u8, u8)> {
        Some(match self {
            T![||] => (1, 2),
            T![&&] => (3, 4),
            T![==] | T![!=] => (5, 6),
            T![<] | T![>] | T![<=] | T![>=] => (7, 8),
            T![+] | T![-] => (9, 10),
            T![*] | T![/] => (11, 12),
            T![^] => (22, 21), // Binds stronger to the left!
            _ => return None,
        })
    }

    fn postfix_binding_power(&self) -> Option<(u8, ())> {
        Some(match self {
            T![!] => (101, ()),
            _ => return None,
        })
    }
}
