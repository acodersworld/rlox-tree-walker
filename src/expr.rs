use crate::token;

pub struct Literal<T> {
    pub value: T
}

pub type LiteralBool = Literal<bool>;
pub type LiteralStr = Literal<String>;
pub type LiteralNumber = Literal<f32>;

pub struct Nil {}

pub struct Binary {
    left: Box<Expr>,
    operator: token::Token,
    right: Box<Expr>
}

pub enum Expr {
    Bool(LiteralBool),
    Str(LiteralStr),
    Number(LiteralNumber),
    Binary(Binary),
    Nil
}

pub trait ExprVisitor<T> {
    fn visit(expr: Expr) -> T;
}
