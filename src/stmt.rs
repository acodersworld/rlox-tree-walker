use crate::expr;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: expr::Expr,
    pub true_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq)]
pub struct Print {
    pub exprs: Vec<expr::Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(expr::Expr),
    Print(Print),
    If(If),
    Block(Block),
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
            Stmt::Block(block) => visitor.visit_block(block),
        }
    }
}

pub fn new_expr(expr: expr::Expr) -> Stmt {
    Stmt::Expr(expr)
}

pub fn new_print(exprs: Vec<expr::Expr>) -> Stmt {
    Stmt::Print(Print { exprs })
}

pub fn new_if(condition: expr::Expr, true_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
    Stmt::If(If {
        condition,
        true_branch: Box::new(true_branch),
        else_branch: else_branch.map(|s| Box::new(s)),
    })
}

pub fn new_block(statements: Vec<Stmt>) -> Stmt {
    Stmt::Block(Block { statements })
}
