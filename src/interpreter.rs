use crate::eval_value::EvalValue;
use crate::expr;
use crate::stmt;
use crate::token::TokenType;

pub struct Interpreter {}

type StmtResult = Result<(), String>;
type EvalResult = Result<EvalValue, String>;
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    fn is_truthy(&self, eval_value: &EvalValue) -> bool {
        let truthy_value = match eval_value {
            EvalValue::Number(n) => *n != 0.0,
            EvalValue::Str(s) => !s.is_empty(),
            EvalValue::Bool(b) => *b,
            EvalValue::Nil => false,
        };

        truthy_value
    }

    pub fn interpret(&self, stmts: &[stmt::Stmt]) -> StmtResult {
        self.execute_many(&stmts)
    }

    pub fn execute(&self, stmt: &stmt::Stmt) -> StmtResult {
        stmt.accept(self)
    }

    pub fn execute_many(&self, stmts: &[stmt::Stmt]) -> StmtResult {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate_expr(&self, expr: &expr::Expr) -> EvalResult {
        return expr.accept(self);
    }
}

impl stmt::StmtVisitor<StmtResult> for Interpreter {
    fn visit_expr(&self, expr: &expr::Expr) -> StmtResult {
        println!("{:#?}", self.evaluate_expr(&expr));
        Ok(())
    }

    fn visit_print(&self, print: &stmt::Print) -> StmtResult {
        for expr in &print.exprs {
            match self.evaluate_expr(&expr) {
                Ok(value) => print!("{} ", value),
                Err(e) => { return Err(e) }
            }
        }
        println!("");
        Ok(())
    }

    fn visit_if(&self, if_ctx: &stmt::If) -> StmtResult {
        let if_cond_result = self.evaluate_expr(&if_ctx.condition)?;
        let is_truthy = self.is_truthy(&if_cond_result);

        if is_truthy {
            self.visit_block(&if_ctx.true_branch)?;
        }
        else if let Some(block) = &if_ctx.else_branch {
            self.visit_block(&block)?;
        }

        Ok(())
    }

    fn visit_block(&self, block: &stmt::Block) -> StmtResult {
        self.execute_many(&block.statements)
    }
}

impl expr::ExprVisitor<EvalResult> for Interpreter {
    fn visit_literal_bool(&self, literal_bool: &bool) -> EvalResult {
        return Ok(EvalValue::Bool(*literal_bool));
    }

    fn visit_literal_str(&self, literal_str: &str) -> EvalResult {
        return Ok(EvalValue::Str(literal_str.to_string()));
    }

    fn visit_literal_number(&self, literal_number: &f32) -> EvalResult {
        return Ok(EvalValue::Number(*literal_number));
    }

    fn visit_binary(&self, binary: &expr::Binary) -> EvalResult {
        let left = self.evaluate_expr(&binary.left)?;
        let right = self.evaluate_expr(&binary.right)?;

        let get_numbers = || -> Result<(f32, f32), String> {
            match (&left, &right) {
                (EvalValue::Number(l), EvalValue::Number(r)) => Ok((*l, *r)),
                _ => Err("Must be numbers".to_owned()),
            }
        };

        match binary.operator.token_type {
            TokenType::Less => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Bool(l < r))
            },
            TokenType::LessEqual => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Bool(l <= r))
            },
            TokenType::Greater => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Bool(l > r))
            },
            TokenType::GreaterEqual => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Bool(l >= r))
            },

            TokenType::EqualEqual => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Bool(l == r))
            },
            TokenType::BangEqual => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Bool(l != r))
            },

            TokenType::And => {
                let left = self.evaluate_expr(&binary.left)?;
                if !self.is_truthy(&left) {
                    return Ok(left);
                }

                return Ok(self.evaluate_expr(&binary.right)?);
            },
            TokenType::Or => {
                let left = self.evaluate_expr(&binary.left)?;
                if self.is_truthy(&left) {
                    return Ok(left);
                }

                return Ok(self.evaluate_expr(&binary.right)?);
            },

            TokenType::Minus => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Number(l - r))
            }
            TokenType::Slash => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Number(l / r))
            }
            TokenType::Star => {
                let (l, r) = get_numbers()?;
                Ok(EvalValue::Number(l * r))
            }
            TokenType::Plus => match (&left, &right) {
                (EvalValue::Number(l), EvalValue::Number(r)) => Ok(EvalValue::Number(l + r)),
                (EvalValue::Str(l), EvalValue::Str(r)) => Ok(EvalValue::Str(l.to_owned() + r)),
                _ => Err("Must be numbers or string".to_owned()),
            },
            _ => Err("Unsupported binary operator".to_owned()),
        }
    }

    fn visit_grouping(&self, grouping: &expr::Expr) -> EvalResult {
        self.evaluate_expr(grouping)
    }

    fn visit_logical_not(&self, expr: &expr::Expr) -> EvalResult {
        let result = self.evaluate_expr(expr)?;
        Ok(EvalValue::Bool(!self.is_truthy(&result)))
    }

    fn visit_unary_negate(&self, expr: &expr::Expr) -> EvalResult {
        let result = self.evaluate_expr(expr)?;
        match result {
            EvalValue::Number(n) => return Ok(EvalValue::Number(-n)),
            _ => return Err("Unary negate expected number".to_owned()),
        }
    }

    fn visit_nil(&self) -> EvalResult {
        return Ok(EvalValue::Nil);
    }
}
