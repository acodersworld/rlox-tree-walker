use std::fmt;

#[derive(Debug)]
pub enum Literal {
    Number(f32),
    Str(String)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Literal::Number(n) => n.to_string(),
            Literal::Str(s) => s.to_string()
        };

        write!(f, "{}", s)
    }
}

