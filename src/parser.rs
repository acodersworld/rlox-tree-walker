use crate::expr;
use crate::stmt;
use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;
use std::vec::Vec;

struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

type ExprResult = Result<expr::Expr, Vec<String>>;
type StmtResult = Result<stmt::Stmt, Vec<String>>;
pub fn parse(tokens: &[Token]) -> Result<Vec<stmt::Stmt>, Vec<String>> {
    let mut parser = Parser {
        iter: tokens.iter().peekable(),
    };

    let mut stmts = vec![];
    while let Some(token) = parser.iter.peek() {
        if token.token_type == TokenType::Eof {
            break;
        }

        stmts.push(parser.statement()?);
    }

    return Ok(stmts);
}

impl<'a> Parser<'a> {
    fn check(&mut self, token_type: &TokenType) -> bool {
        if let Some(t) = self.iter.peek() {
            return t.token_type == *token_type;
        }

        return false;
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types {
            if self.check(token_type) {
                return Some(self.iter.next().unwrap().clone());
            }
        }

        return None;
    }

    fn consume_token(
        &mut self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<(), Vec<String>> {
        if self.check(&token_type) {
            self.iter.next();
            return Ok(());
        }

        if let Some(token) = self.iter.peek() {
            Err(vec![format!(
                "Line {} at '{}': {}",
                token.line,
                (**token).to_string(),
                error_message.to_string()
            )])
        } else {
            Err(vec![format!("At EOF: {}", error_message.to_string())])
        }
    }

    fn statement(&mut self) -> StmtResult {
        if let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::If => {
                    self.iter.next();
                    return self.if_stmt();
                }
                TokenType::LeftBrace => {
                    self.iter.next();
                    return self.block_stmt();
                }
                TokenType::Print => {
                    self.iter.next();
                    return self.print_stmt();
                }
                TokenType::Var => {
                    self.iter.next();
                    return self.var_stmt();
                }
                _ => {}
            }
        }

        return self.expr_stmt();
    }

    fn expr_stmt(&mut self) -> StmtResult {
        let expr = self.expression()?;

        self.consume_token(TokenType::SemiColon, "Expected ';' after expression")?;

        Ok(stmt::new_expr(expr))
    }

    fn if_stmt(&mut self) -> StmtResult {
        self.consume_token(TokenType::LeftParen, "Expected '(' after if")?;
        let condition = self.expression()?;
        self.consume_token(TokenType::RightParen, "Expected ')' after if condition")?;

        let true_branch = self.statement()?;

        let mut else_branch = None;
        if self.match_tokens(&[TokenType::Else]).is_some() {
            else_branch = Some(self.statement()?);
        }

        Ok(stmt::new_if(condition, true_branch, else_branch))
    }

    fn block_stmt(&mut self) -> StmtResult {
        let mut statements = vec![];

        while self.match_tokens(&[TokenType::RightBrace]).is_none() {
            statements.push(self.statement()?);
        }

        Ok(stmt::new_block(statements))
    }

    fn print_stmt(&mut self) -> StmtResult {
        let mut exprs = vec![self.expression()?];

        while self.match_tokens(&[TokenType::Comma]).is_some() {
            exprs.push(self.expression()?);
        }

        self.consume_token(TokenType::SemiColon, "Expected ';' after print statement")?;

        Ok(stmt::new_print(exprs))
    }

    fn var_stmt(&mut self) -> StmtResult {
        let (identifier_name, line) = {
            let token = match self.iter.next() {
                Some(t) => t,
                None => return Err(vec![format!("Expected identifier after 'var'")]),
            };

            match &token.token_type {
                TokenType::Identifier(name) => (name, token.line),
                _ => return Err(vec![format!("Expected identifier after 'var'")]),
            }
        };

        self.consume_token(TokenType::Equal, "Expected '=' after var identifier")?;

        let expr = self.expression()?;

        self.consume_token(TokenType::SemiColon, "Expected ';' after print statement")?;

        Ok(stmt::new_var(identifier_name, line, expr))
    }

    fn expression(&mut self) -> ExprResult {
        return self.assignment();
    }

    fn assignment(&mut self) -> ExprResult {
        let mut expr = self.logical_or()?;

        if self.match_tokens(&[TokenType::Equal]).is_some() {
            match &expr {
                expr::Expr::Variable(variable) => {
                    expr = expr::new_assignment(&variable.name, variable.line, self.expression()?);
                }
                _ => return Err(vec![format!("Invalid assignment target")]),
            }
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> ExprResult {
        let expr = self.logical_and()?;

        if let Some(operator) = self.match_tokens(&[TokenType::Or]) {
            return Ok(expr::new_binary(expr, operator, self.comparison()?));
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> ExprResult {
        let expr = self.equality()?;

        if let Some(operator) = self.match_tokens(&[TokenType::And]) {
            return Ok(expr::new_binary(expr, operator, self.comparison()?));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ExprResult {
        let expr = self.comparison()?;

        if let Some(operator) = self.match_tokens(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            match operator.token_type {
                TokenType::EqualEqual | TokenType::BangEqual => {
                    return Ok(expr::new_binary(expr, operator, self.comparison()?));
                }
                _ => panic!("Unexpected token parsing comparison: {:?}", operator),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ExprResult {
        let expr = self.term()?;

        if let Some(operator) = self.match_tokens(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            match operator.token_type {
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual => {
                    return Ok(expr::new_binary(expr, operator, self.term()?))
                }
                _ => panic!("Unexpected token parsing comparison: {:?}", operator),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;

        while let Some(token_type) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let binary = expr::new_binary(expr, token_type, self.factor()?);

            expr = binary;
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ExprResult {
        let mut expr = self.unary()?;

        while let Some(token_type) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let binary = expr::new_binary(expr, token_type, self.unary()?);

            expr = binary;
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> ExprResult {
        if let Some(t) = self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            match t.token_type {
                TokenType::Bang => {
                    let expr = self.unary()?;
                    return Ok(expr::new_logical_not(expr));
                }
                TokenType::Minus => {
                    let expr = self.unary()?;
                    return Ok(expr::new_unary_negate(expr));
                }
                _ => panic!("Unexpected token parsing unary: {:?}", t),
            }
        }

        return self.primary();
    }

    fn grouping(&mut self) -> ExprResult {
        let expr = self.expression()?;
        if let Some(t) = self.iter.next() {
            if t.token_type == TokenType::RightParen {
                return Ok(expr::new_grouping(expr));
            } else {
                return Err(vec![format!(
                    "Expected ')' but found {} at line {}",
                    t.to_string(),
                    t.line
                )]);
            }
        } else {
            return Err(vec![format!("Expected ')' but found EOF")]);
        }
    }

    fn identifier(&mut self, name: &str, line: u32) -> ExprResult {
        Ok(expr::new_variable(name, line))
    }

    fn primary(&mut self) -> ExprResult {
        if let Some(t) = self.iter.next() {
            match &t.token_type {
                TokenType::True => return Ok(expr::Expr::Bool(true)),
                TokenType::False => return Ok(expr::Expr::Bool(false)),

                TokenType::Nil => return Ok(expr::Expr::Nil),

                TokenType::Number(value) => return Ok(expr::Expr::Number(*value)),
                TokenType::Str(value) => return Ok(expr::Expr::Str(value.clone())),

                TokenType::LeftParen => return self.grouping(),

                TokenType::Identifier(name) => return self.identifier(&name, t.line),

                _ => {
                    return Err(vec![format!(
                        "Expected primary expression, found {} at line {}",
                        t.to_string(),
                        t.line
                    )])
                }
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
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::True, 1),
                Token::new(TokenType::SemiColon, 1)
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::Expr::Bool(true))]
        );
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::False, 1),
                Token::new(TokenType::SemiColon, 1)
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::Expr::Bool(false))]
        );
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Nil, 1),
                Token::new(TokenType::SemiColon, 1)
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::Expr::Nil)]
        );
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(3.142), 1),
                Token::new(TokenType::SemiColon, 1)
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::Expr::Number(3.142))]
        );
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Str("Hello World".to_owned()), 1),
                Token::new(TokenType::SemiColon, 1)
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::Expr::Str("Hello World".to_owned()))]
        );
    }

    #[test]
    fn term() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(4.0), 1),
                Token::new(TokenType::Plus, 1),
                Token::new(TokenType::Number(8.5), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(4.0),
                Token::new(TokenType::Plus, 1),
                expr::Expr::Number(8.5)
            ))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(4.0), 1),
                Token::new(TokenType::Minus, 1),
                Token::new(TokenType::Number(8.5), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(4.0),
                Token::new(TokenType::Minus, 1),
                expr::Expr::Number(8.5)
            ))]
        );
    }

    #[test]
    fn factor() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(4.0), 1),
                Token::new(TokenType::Slash, 1),
                Token::new(TokenType::Number(8.5), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(4.0),
                Token::new(TokenType::Slash, 1),
                expr::Expr::Number(8.5)
            ))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(4.0), 1),
                Token::new(TokenType::Star, 1),
                Token::new(TokenType::Number(8.5), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(4.0),
                Token::new(TokenType::Star, 1),
                expr::Expr::Number(8.5)
            ))]
        );
    }

    #[test]
    fn term_factor() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::Plus, 1),
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::Slash, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::Minus, 1),
                Token::new(TokenType::Number(4.0), 1),
                Token::new(TokenType::Star, 1),
                Token::new(TokenType::Number(5.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::new_binary(
                    expr::Expr::Number(1.0),
                    Token::new(TokenType::Plus, 1),
                    expr::new_binary(
                        expr::Expr::Number(2.0),
                        Token::new(TokenType::Slash, 1),
                        expr::Expr::Number(3.0)
                    )
                ),
                Token::new(TokenType::Minus, 1),
                expr::new_binary(
                    expr::Expr::Number(4.0),
                    Token::new(TokenType::Star, 1),
                    expr::Expr::Number(5.0)
                )
            ))]
        );
    }

    #[test]
    fn error_cases() {
        assert!(parse(&vec![Token::new(TokenType::Star, 1)]).is_err());
        assert!(parse(&vec![
            Token::new(TokenType::Number(8.0), 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::SemiColon, 1),
        ])
        .is_err());
        assert!(parse(&vec![
            Token::new(TokenType::Number(8.0), 1),
            Token::new(TokenType::Number(1.0), 1)
        ])
        .is_err());
    }

    #[test]
    fn grouping() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::Plus, 1),
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::Star, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::Minus, 1),
                Token::new(TokenType::Number(4.0), 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::new_grouping(expr::new_binary(
                    expr::Expr::Number(1.0),
                    Token::new(TokenType::Plus, 1),
                    expr::Expr::Number(2.0)
                )),
                Token::new(TokenType::Star, 1),
                expr::new_grouping(expr::new_binary(
                    expr::Expr::Number(3.0),
                    Token::new(TokenType::Minus, 1),
                    expr::Expr::Number(4.0)
                ))
            ))]
        );
    }

    #[test]
    fn grouping_unmatched_parentheis() {
        let result = parse(&vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(2.0), 1),
            Token::new(TokenType::SemiColon, 1),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn bang() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Bang, 1),
                Token::new(TokenType::True, 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_logical_not(expr::Expr::Bool(
                true
            )))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Bang, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::Plus, 1),
                Token::new(TokenType::Number(5.0), 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_logical_not(expr::new_grouping(
                expr::new_binary(
                    expr::Expr::Number(2.0),
                    Token::new(TokenType::Plus, 1),
                    expr::Expr::Number(5.0)
                )
            )))]
        )
    }

    #[test]
    fn unary_negate() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Minus, 1),
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_unary_negate(expr::Expr::Number(
                2.0
            )))]
        );

        assert!(parse(&vec![Token::new(TokenType::Minus, 1)]).is_err());
    }

    #[test]
    fn comparison() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::Less, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(2.0),
                Token::new(TokenType::Less, 1),
                expr::Expr::Number(3.0)
            ))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::LessEqual, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(2.0),
                Token::new(TokenType::LessEqual, 1),
                expr::Expr::Number(3.0)
            ))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::Greater, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(2.0),
                Token::new(TokenType::Greater, 1),
                expr::Expr::Number(3.0)
            ))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::GreaterEqual, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(2.0),
                Token::new(TokenType::GreaterEqual, 1),
                expr::Expr::Number(3.0)
            ))]
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::EqualEqual, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(3.0),
                Token::new(TokenType::EqualEqual, 1),
                expr::Expr::Number(3.0)
            ))]
        );

        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::BangEqual, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(2.0),
                Token::new(TokenType::BangEqual, 1),
                expr::Expr::Number(3.0)
            ))]
        );
    }

    #[test]
    fn logical_or() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::Or, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(3.0),
                Token::new(TokenType::Or, 1),
                expr::Expr::Number(3.0)
            ))]
        );
    }

    #[test]
    fn logical_and() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::And, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(expr::new_binary(
                expr::Expr::Number(2.0),
                Token::new(TokenType::And, 1),
                expr::Expr::Number(3.0)
            ))]
        );
    }

    #[test]
    fn print() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::Comma, 1),
                Token::new(TokenType::Str("Hello, ".to_owned()), 1),
                Token::new(TokenType::Comma, 1),
                Token::new(TokenType::Str("World".to_owned()), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_print(vec![
                expr::Expr::Number(3.0),
                expr::Expr::Str("Hello, ".to_owned()),
                expr::Expr::Str("World".to_owned())
            ])]
        );
    }

    #[test]
    fn test_if_no_else_no_braces() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::If, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::True, 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_if(
                expr::Expr::Bool(true),
                stmt::new_print(vec![expr::Expr::Number(1.0)]),
                None
            )]
        );
    }

    #[test]
    fn test_if_no_else() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::If, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::True, 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::LeftBrace, 1),
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::SemiColon, 1),
                Token::new(TokenType::RightBrace, 1),
            ])
            .unwrap(),
            vec![stmt::new_if(
                expr::Expr::Bool(true),
                stmt::new_block(vec![stmt::new_print(vec![expr::Expr::Number(1.0)])]),
                None
            )]
        );
    }

    #[test]
    fn test_if_with_else_no_braces() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::If, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::True, 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::SemiColon, 1),
                Token::new(TokenType::Else, 1),
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_if(
                expr::Expr::Bool(true),
                stmt::new_print(vec![expr::Expr::Number(1.0)]),
                Some(stmt::new_print(vec![expr::Expr::Number(2.0)]))
            )]
        );
    }

    #[test]
    fn test_if_with_else() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::If, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::True, 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::LeftBrace, 1),
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::SemiColon, 1),
                Token::new(TokenType::RightBrace, 1),
                Token::new(TokenType::Else, 1),
                Token::new(TokenType::LeftBrace, 1),
                Token::new(TokenType::Print, 1),
                Token::new(TokenType::Number(2.0), 1),
                Token::new(TokenType::SemiColon, 1),
                Token::new(TokenType::RightBrace, 1),
            ])
            .unwrap(),
            vec![stmt::new_if(
                expr::Expr::Bool(true),
                stmt::new_block(vec![stmt::new_print(vec![expr::Expr::Number(1.0)])]),
                Some(stmt::new_block(vec![stmt::new_print(vec![
                    expr::Expr::Number(2.0)
                ])]))
            )]
        );
    }

    #[test]
    fn test_var() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Var, 1),
                Token::new(TokenType::Identifier("variable".to_owned()), 1),
                Token::new(TokenType::Equal, 1),
                Token::new(TokenType::Number(10.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_var("variable", 1, expr::Expr::Number(10.0))]
        );
    }

    #[test]
    fn test_assignment() {
        assert_eq!(
            parse(&vec![
                Token::new(TokenType::Identifier("variable".to_owned()), 1),
                Token::new(TokenType::Equal, 1),
                Token::new(TokenType::Number(10.0), 1),
                Token::new(TokenType::SemiColon, 1),
            ])
            .unwrap(),
            vec![stmt::new_expr(
                expr::new_assignment("variable", 1, expr::Expr::Number(10.0))
            )]
        );
    }
}
