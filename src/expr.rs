use crate::token;

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Bool(bool),
    Str(String),
    Number(f32),
    Binary(Binary),
    Nil
}

pub trait ExprVisitor<T> {
    fn visit_literal_bool(&self, literal_bool: &bool) -> T;
    fn visit_literal_str(&self, literal_str: &str) -> T;
    fn visit_literal_number(&self, literal_number: &f32) -> T;
    fn visit_binary(&self, binary: &Binary) -> T;
	fn visit_nil(&self) -> T;
}

impl Expr {
	fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> T {
		match self {
			Expr::Bool(b) => visitor.visit_literal_bool(b),
			Expr::Str(s) => visitor.visit_literal_str(s),
			Expr::Number(n) => visitor.visit_literal_number(n),
			Expr::Binary(b) => visitor.visit_binary(b),
			Expr::Nil => visitor.visit_nil()
		}
	}
}


