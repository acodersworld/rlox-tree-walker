use crate::expr;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: expr::Expr,
    pub true_branch: Block,
    pub else_branch: Option<Block>
}

#[derive(Debug, PartialEq)]
pub struct Print {
    pub exprs: Vec<expr::Expr>
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(expr::Expr),
    Print(Print),
    If(If),
    Block(Block)
}

pub trait StmtVisitor<T> {
    fn visit_expr(&self, expr: &expr::Expr) -> T;
    fn visit_print(&self, print: &Print) -> T;
    fn visit_if(&self, if_cxt: &If) -> T;
    fn visit_block(&self, block: &Block) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &impl StmtVisitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(print) => visitor.visit_print(print),
            Stmt::If(if_ctx) => visitor.visit_if(if_ctx),
            Stmt::Block(block) => visitor.visit_block(block)
        }
    }
}

