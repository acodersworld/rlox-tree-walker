use std::vec::Vec;
use std::slice::Iter;
use std::iter::Peekable;
use crate::token::{Token, TokenType};
use crate::expr;

struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>
}

type ExprResult = Result<expr::Expr, std::vec::Vec<String>>;
pub fn parse(tokens: &Vec<Token>) -> ExprResult {
    let mut parser = Parser {
        iter: tokens.iter().peekable()
    };

    while parser.iter.peek().is_some() {
        return parser.expression();
    }

    unreachable!()
}

impl<'a> Parser<'a> {

    fn check(&mut self, token_type: &TokenType) -> bool {
        if let Some(t) = self.iter.peek() {
            return t.token_type == *token_type
        }

        return false
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types {
            if self.check(token_type) {
                return Some(self.iter.next().unwrap().clone());
            }
        }

        return None
    }

    fn expression(&mut self) -> ExprResult {
        return self.term()
    }

    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;

        while let Some(token_type) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let binary = expr::Expr::Binary(expr::Binary{
                left: Box::new(expr),
                operator: token_type,
                right: Box::new(self.factor()?)
            });

            expr = binary;
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ExprResult {
        let mut expr = self.primary()?;

        while let Some(token_type) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let binary = expr::Expr::Binary(expr::Binary{
                left: Box::new(expr),
                operator: token_type,
                right: Box::new(self.primary()?)
            });

            expr = binary;
        }

        return Ok(expr);
    }

    fn primary(&mut self) -> ExprResult {
        if let Some(t) = self.iter.next() {
            match &t.token_type {
                TokenType::True => return Ok(expr::Expr::Bool(expr::LiteralBool{value: true})),
                TokenType::False => return Ok(expr::Expr::Bool(expr::LiteralBool{value: false})),

                TokenType::Nil => return Ok(expr::Expr::Nil),

                TokenType::Number(value) => return Ok(expr::Expr::Number(expr::LiteralNumber{value: *value})),
                TokenType::Str(value) => return Ok(expr::Expr::Str(expr::LiteralStr{value: value.clone()})),

                _ => return Err(vec![format!("Expected primary expression, found {}", t.to_string())])
            };
        }

        Err(vec!["Expected primary expression, found EOF".to_owned()])
    }
}

