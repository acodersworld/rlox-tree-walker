use std::vec::Vec;
use std::slice::Iter;
use std::iter::Peekable;
use crate::token::{Token, TokenType};
use crate::expr;

struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>
}

type ExprResult = Result<expr::Expr, String>;
fn parse(tokens: &Vec<Token>) -> ExprResult {
    let mut parser = Parser {
        iter: tokens.iter().peekable()
    };

    while parser.iter.peek().is_some() {
        return parser.expression();
    }

    unreachable!()
}

impl<'a> Parser<'a> {

    fn check(&mut self, token_type: TokenType) -> bool {
        if let Some(t) = self.iter.peek() {
            return t.token_type == token_type
        }

        return false
    }

    fn expression(&mut self) -> ExprResult {
        return self.primary()
    }

    fn primary(&mut self) -> ExprResult {
        if let Some(t) = self.iter.peek() {
            match &t.token_type {
                TokenType::True => return Ok(expr::Expr::Bool(expr::LiteralBool{value: true})),
                TokenType::False => return Ok(expr::Expr::Bool(expr::LiteralBool{value: false})),

                TokenType::Nil => return Ok(expr::Expr::Nil),

                TokenType::Number(value) => return Ok(expr::Expr::Number(expr::LiteralNumber{value: *value})),
                TokenType::Str(value) => return Ok(expr::Expr::Str(expr::LiteralStr{value: value.clone()})),

                _ => return Err("".to_owned())
            };
        }

        Err("".to_owned())
    }
}

