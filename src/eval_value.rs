use std::fmt;

#[derive(Debug, Clone)]
pub enum EvalValue {
    Number(f32),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for EvalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::Number(n) => write!(f, "{}", n),
            EvalValue::Str(s) => write!(f, "{}", s),
            EvalValue::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            EvalValue::Nil => write!(f, "nil"),
        }
    }
}
