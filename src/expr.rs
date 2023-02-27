use crate::token;
use std::cell::Cell;

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub line: u32,
    pub stack_idx: Cell<Option<usize>>
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub target: String,
    pub line: u32,
    pub expr: Box<Expr>,
    pub stack_idx: Cell<Option<usize>>
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub line: u32,
    pub arguments: Vec<Box<Expr>>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Bool(bool),
    Str(String),
    Number(f32),
    Binary(Binary),
    Grouping(Box<Expr>),
    LogicalNot(Box<Expr>),
    UnaryNegate(Box<Expr>),
    Variable(Variable),
    Assignment(Assignment),
    Call(Call),
    Nil,
}

pub trait ExprVisitor<T> {
    fn visit_literal_bool(&self, literal_bool: &bool) -> T;
    fn visit_literal_str(&self, literal_str: &str) -> T;
    fn visit_literal_number(&self, literal_number: &f32) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_grouping(&mut self, grouping: &Expr) -> T;
    fn visit_logical_not(&mut self, expr: &Expr) -> T;
    fn visit_unary_negate(&mut self, expr: &Expr) -> T;
    fn visit_variable(&mut self, variable: &Variable) -> T;
    fn visit_assignment(&mut self, assignment: &Assignment) -> T;
    fn visit_call(&mut self, call: &Call) -> T;
    fn visit_nil(&self) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        match self {
            Expr::Bool(b) => visitor.visit_literal_bool(b),
            Expr::Str(s) => visitor.visit_literal_str(s),
            Expr::Number(n) => visitor.visit_literal_number(n),
            Expr::Binary(b) => visitor.visit_binary(b),
            Expr::Grouping(g) => visitor.visit_grouping(g),
            Expr::LogicalNot(ln) => visitor.visit_logical_not(ln),
            Expr::UnaryNegate(un) => visitor.visit_unary_negate(un),
            Expr::Variable(v) => visitor.visit_variable(&v),
            Expr::Assignment(v) => visitor.visit_assignment(&v),
            Expr::Call(v) => visitor.visit_call(&v),
            Expr::Nil => visitor.visit_nil(),
        }
    }
}

pub fn new_binary(left: Expr, operator: token::Token, right: Expr) -> Expr {
    Expr::Binary(Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
}

pub fn new_grouping(expr: Expr) -> Expr {
    Expr::Grouping(Box::new(expr))
}

pub fn new_logical_not(expr: Expr) -> Expr {
    Expr::LogicalNot(Box::new(expr))
}

pub fn new_unary_negate(expr: Expr) -> Expr {
    Expr::UnaryNegate(Box::new(expr))
}

pub fn new_variable(name: &str, line: u32) -> Expr {
    Expr::Variable(Variable {
        name: name.to_string(),
        line,
        stack_idx: Cell::new(None)
    })
}

pub fn new_assignment(target: &str, line: u32, expr: Expr) -> Expr {
    Expr::Assignment(Assignment {
        target: target.to_string(),
        line,
        expr: Box::new(expr),
        stack_idx: Cell::new(None)
    })
}

pub fn new_call(callee: Expr, line: u32, arguments: Vec<Box<Expr>>) -> Expr {
    Expr::Call(Call {
        callee: Box::new(callee),
        line,
        arguments,
    })
}
