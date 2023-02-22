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
pub struct Var {
    pub name: String,
    pub line: u32,
    pub initializer: expr::Expr,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: expr::Expr,
    pub body: Box<Stmt>
}

#[derive(Debug, PartialEq)]
pub struct For {
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<expr::Expr>,
    pub loop_eval: Option<expr::Expr>,
    pub body: Box<Stmt>
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(expr::Expr),
    Print(Print),
    If(If),
    Block(Block),
    Var(Var),
    While(While),
}

pub trait StmtVisitor<T> {
    fn visit_expr(&mut self, expr: &expr::Expr) -> T;
    fn visit_print(&mut self, print: &Print) -> T;
    fn visit_if(&mut self, if_cxt: &If) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_var(&mut self, var: &Var) -> T;
    fn visit_while(&mut self, while_ctx: &While) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(print) => visitor.visit_print(print),
            Stmt::If(if_ctx) => visitor.visit_if(if_ctx),
            Stmt::Block(block) => visitor.visit_block(block),
            Stmt::Var(var) => visitor.visit_var(var),
            Stmt::While(while_ctx) => visitor.visit_while(while_ctx),
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

pub fn new_var(name: &str, line: u32, initializer: expr::Expr) -> Stmt {
    Stmt::Var(Var {
        name: name.to_string(),
        line,
        initializer
    })
}

pub fn new_while(condition: expr::Expr, body: Stmt) -> Stmt {
    Stmt::While(While{
        condition,
        body: Box::new(body)
    })
}

