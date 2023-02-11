use crate::expr;
use crate::token::TokenType;
use crate::eval_value::EvalValue;

pub struct Interpreter {
}

type EvalResult = Result<EvalValue, String>;
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn evaluate_expr(&self, expr: &expr::Expr) -> EvalResult {
        return expr.accept(self);
    }

}

impl expr::ExprVisitor<EvalResult> for Interpreter {
    fn visit_literal_bool(&self, literal_bool: &bool) -> EvalResult {
        return Ok(EvalValue::Bool(*literal_bool))
    }

    fn visit_literal_str(&self, literal_str: &str) -> EvalResult {
        return Ok(EvalValue::Str(literal_str.to_string()))
    }

    fn visit_literal_number(&self, literal_number: &f32) -> EvalResult {
        return Ok(EvalValue::Number(*literal_number))
    }

    fn visit_binary(&self, binary: &expr::Binary) -> EvalResult {
        let left = self.evaluate_expr(&binary.left)?;
        let right = self.evaluate_expr(&binary.right)?;

        let get_numbers = || -> Result<(f32, f32), String> {
            match (&left, &right) {
                (EvalValue::Number(l), EvalValue::Number(r)) => Ok((*l,*r)),
                _ => Err("Must be numbers".to_owned())
            }
        };

        match binary.operator.token_type {
            TokenType::Minus => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Number(l - r))
            },
            TokenType::Slash => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Number(l / r))
            },
            TokenType::Star => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Number(l * r))
            },
            TokenType::Plus => {
                match (&left, &right) {
                    (EvalValue::Number(l), EvalValue::Number(r)) => Ok(EvalValue::Number(l + r)),
                    (EvalValue::Str(l), EvalValue::Str(r)) => Ok(EvalValue::Str(l.to_owned() + r)),
                    _ => Err("Must be numbers or string".to_owned())
                }
            },
            _ => Err("Unsupported binary operator".to_owned())
        }
    }

	fn visit_nil(&self) -> EvalResult {
        return Ok(EvalValue::Nil)
    }
}
