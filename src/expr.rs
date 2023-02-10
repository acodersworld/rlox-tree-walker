use crate::token;

#[derive(Debug)]
pub struct Literal<T> {
    pub value: T
}

pub type LiteralBool = Literal<bool>;
pub type LiteralStr = Literal<String>;
pub type LiteralNumber = Literal<f32>;

#[derive(Debug)]
pub struct Nil {}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>
}

#[derive(Debug)]
pub enum Expr {
    Bool(LiteralBool),
    Str(LiteralStr),
    Number(LiteralNumber),
    Binary(Binary),
    Nil
}

pub trait ExprVisitor<T> {
    fn visit_literal_bool(&self, literal_bool: &LiteralBool) -> T;
    fn visit_literal_str(&self, literal_str: &LiteralStr) -> T;
    fn visit_literal_number(&self, literal_number: &LiteralNumber) -> T;
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


