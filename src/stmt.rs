use crate::expr;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(expr::Expr),
    Print(Vec<expr::Expr>)
}

