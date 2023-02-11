use std::vec::Vec;
use std::slice::Iter;
use std::iter::Peekable;
use crate::token::{Token, TokenType};
use crate::expr;

struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>
}

type ExprResult = Result<expr::Expr, std::vec::Vec<String>>;
pub fn parse(tokens: &[Token]) -> ExprResult {
    let mut parser = Parser {
        iter: tokens.iter().peekable()
    };

    let expr = parser.expression();
    if parser.iter.peek().is_some() {
        return Err(vec!["Expected EOF".to_string()]);
    }

    return expr;
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
                TokenType::True => return Ok(expr::Expr::Bool(true)),
                TokenType::False => return Ok(expr::Expr::Bool(false)),

                TokenType::Nil => return Ok(expr::Expr::Nil),

                TokenType::Number(value) => return Ok(expr::Expr::Number(*value)),
                TokenType::Str(value) => return Ok(expr::Expr::Str(value.clone())),

                _ => return Err(vec![format!("Expected primary expression, found {}", t.to_string())])
            };
        }

        Err(vec!["Expected primary expression, found EOF".to_owned()])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primary() {
       assert_eq!(parse(&vec![Token::new(TokenType::True, 1)]).unwrap(), expr::Expr::Bool(true));
       assert_eq!(parse(&vec![Token::new(TokenType::False, 1)]).unwrap(), expr::Expr::Bool(false));
       assert_eq!(parse(&vec![Token::new(TokenType::Nil, 1)]).unwrap(), expr::Expr::Nil);
       assert_eq!(parse(&vec![Token::new(TokenType::Number(3.142), 1)]).unwrap(), expr::Expr::Number(3.142));
       assert_eq!(parse(&vec![Token::new(TokenType::Str("Hello World".to_owned()), 1)]).unwrap(), expr::Expr::Str("Hello World".to_owned()));
    }

    #[test]
    fn term() {
       assert_eq!(
           parse(
               &vec![
                   Token::new(TokenType::Number(4.0), 1),
                   Token::new(TokenType::Plus, 1),
                   Token::new(TokenType::Number(8.5), 1)
               ]).unwrap(), 
           expr::Expr::Binary(expr::Binary{
               left: Box::new(expr::Expr::Number(4.0)),
               operator: Token::new(TokenType::Plus, 1),
               right: Box::new(expr::Expr::Number(8.5))
           }));

       assert_eq!(
           parse(
               &vec![
                   Token::new(TokenType::Number(4.0), 1),
                   Token::new(TokenType::Minus, 1),
                   Token::new(TokenType::Number(8.5), 1)
               ]).unwrap(), 
           expr::Expr::Binary(expr::Binary{
               left: Box::new(expr::Expr::Number(4.0)),
               operator: Token::new(TokenType::Minus, 1),
               right: Box::new(expr::Expr::Number(8.5))
           }));
    }

    #[test]
    fn factor() {
       assert_eq!(
           parse(
               &vec![
                   Token::new(TokenType::Number(4.0), 1),
                   Token::new(TokenType::Slash, 1),
                   Token::new(TokenType::Number(8.5), 1)
               ]).unwrap(), 
           expr::Expr::Binary(expr::Binary{
               left: Box::new(expr::Expr::Number(4.0)),
               operator: Token::new(TokenType::Slash, 1),
               right: Box::new(expr::Expr::Number(8.5))
           }));

       assert_eq!(
           parse(
               &vec![
                   Token::new(TokenType::Number(4.0), 1),
                   Token::new(TokenType::Star, 1),
                   Token::new(TokenType::Number(8.5), 1)
               ]).unwrap(), 
           expr::Expr::Binary(expr::Binary{
               left: Box::new(expr::Expr::Number(4.0)),
               operator: Token::new(TokenType::Star, 1),
               right: Box::new(expr::Expr::Number(8.5))
           }));
    }

    #[test]
    fn term_factor() {
        assert_eq!(
           parse(
               &vec![
                   Token::new(TokenType::Number(1.0), 1),
                   Token::new(TokenType::Plus, 1),
                   Token::new(TokenType::Number(2.0), 1),
                   Token::new(TokenType::Slash, 1),
                   Token::new(TokenType::Number(3.0), 1),
                   Token::new(TokenType::Minus, 1),
                   Token::new(TokenType::Number(4.0), 1),
                   Token::new(TokenType::Star, 1),
                   Token::new(TokenType::Number(5.0), 1)
               ]).unwrap(), 

           expr::Expr::Binary(expr::Binary{
               left: 
                    Box::new(expr::Expr::Binary(expr::Binary{
                        left: Box::new(expr::Expr::Number(1.0)),
                        operator: Token::new(TokenType::Plus, 1),
                        right: Box::new(expr::Expr::Binary(expr::Binary{
                           left: Box::new(expr::Expr::Number(2.0)),
                           operator: Token::new(TokenType::Slash, 1),
                           right: Box::new(expr::Expr::Number(3.0))
                       }))
                    })),
               operator: Token::new(TokenType::Minus, 1),
               right: 
                    Box::new(expr::Expr::Binary(expr::Binary{
                        left: Box::new(expr::Expr::Number(4.0)),
                        operator: Token::new(TokenType::Star, 1),
                        right: Box::new(expr::Expr::Number(5.0))
                   }))
            })
        );
    }

    #[test]
    fn error_cases() {
           assert!(parse(&vec![Token::new(TokenType::Star, 1)]).is_err());
           assert!(parse(&vec![Token::new(TokenType::Number(8.0), 1), Token::new(TokenType::Star, 1)]).is_err());
           assert!(parse(&vec![Token::new(TokenType::Number(8.0), 1), Token::new(TokenType::Number(1.0), 1)]).is_err());
    }
}
