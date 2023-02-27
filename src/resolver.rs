use crate::environment::Environment;
use crate::stmt;
use crate::expr;
use crate::eval_value::EvalValue;
use std::rc::Rc;

pub struct Resolver {
    pub local_environments: Vec<Box<Environment>>,
}

type StmtResult = Result<(), String>;
type EvalResult = Result<(), String>;
impl<'a> Resolver {
    pub fn new() -> Resolver {
        Resolver {
            local_environments: vec![]
        }
    }

    pub fn resolve(&mut self, stmts: &[stmt::Stmt]) -> StmtResult {
        self.execute_many(&stmts)
    }

    pub fn execute(&mut self, stmt: &stmt::Stmt) -> StmtResult {
        stmt.accept(self)
    }

    pub fn execute_many(&mut self, stmts: &[stmt::Stmt]) -> StmtResult {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate_expr(&mut self, expr: &expr::Expr) -> EvalResult {
        return expr.accept(self);
    }
}

impl stmt::StmtVisitor<StmtResult> for Resolver {
    fn visit_expr(&mut self, expr: &expr::Expr) -> StmtResult {
        self.evaluate_expr(&expr)?;
        Ok(())
    }

    fn visit_print(&mut self, print: &stmt::Print) -> StmtResult {
        for expr in &print.exprs {
            self.evaluate_expr(&expr)?;
        }
        Ok(())
    }

    fn visit_if(&mut self, if_ctx: &stmt::If) -> StmtResult {
        self.evaluate_expr(&if_ctx.condition)?;
        self.execute(&if_ctx.true_branch)?;
        if let Some(branch) = &if_ctx.else_branch {
            self.execute(&branch)?;
        }

        Ok(())
    }

    fn visit_block(&mut self, block: &stmt::Block) -> StmtResult {
        if self.local_environments.is_empty() {
            self.local_environments.push(Box::new(Environment::new()));
            self.execute_many(&block.statements)?;
            self.local_environments.pop();
        }
        else {
            self.local_environments.last_mut().unwrap().push_scope();
            self.execute_many(&block.statements)?;
            self.local_environments.last_mut().unwrap().pop_scope();
        }

        Ok(())
    }

    fn visit_var(&mut self, var: &stmt::Var) -> StmtResult {
        self.evaluate_expr(&var.initializer)?;

        if let Some(local_environment) = self.local_environments.last_mut() {
            local_environment.define_var(&var.name, EvalValue::Nil);
        }

        Ok(())
    }

    fn visit_while(&mut self, while_ctx: &stmt::While) -> StmtResult {
        self.evaluate_expr(&while_ctx.condition)?;
        self.execute(&while_ctx.body)?;
        Ok(())
    }

    fn visit_function(&mut self, function: &Rc<stmt::Function>) -> StmtResult {
        let env = {
            if let Some(local_environment) = self.local_environments.last_mut() {
                //TODO: Implement recursion. Can't store function in it's own closure. Causes
                //reference cycle.
                //local_environment.define_var(&function.name, EvalValue::Nil);
                Environment::new_capture_env(local_environment)
            }
            else {
                Environment::new()
            }
        };
        
        self.local_environments.push(Box::new(env));
        self.execute_many(&function.statements)?;
        self.local_environments.pop();
        Ok(())
    }

    fn visit_return(&mut self, expr: &expr::Expr) -> StmtResult {
        self.evaluate_expr(expr)?;
        Ok(())
    }
}

impl expr::ExprVisitor<EvalResult> for Resolver {
    fn visit_literal_bool(&self, _literal_bool: &bool) -> EvalResult {
        return Ok(());
    }

    fn visit_literal_str(&self, _literal_str: &str) -> EvalResult {
        return Ok(());
    }

    fn visit_literal_number(&self, _literal_number: &f32) -> EvalResult {
        return Ok(());
    }

    fn visit_binary(&mut self, binary: &expr::Binary) -> EvalResult {
        self.evaluate_expr(&binary.left)?;
        self.evaluate_expr(&binary.right)?;
        Ok(())
    }

    fn visit_grouping(&mut self, grouping: &expr::Expr) -> EvalResult {
        self.evaluate_expr(grouping)
    }

    fn visit_logical_not(&mut self, expr: &expr::Expr) -> EvalResult {
        self.evaluate_expr(expr)?;
        Ok(())
    }

    fn visit_unary_negate(&mut self, expr: &expr::Expr) -> EvalResult {
        self.evaluate_expr(expr)?;
        Ok(())
    }

    fn visit_variable(&mut self, variable: &expr::Variable) -> EvalResult {
        if let Some(local_environment) = &self.local_environments.last() {
            if let Some(idx) = local_environment.get_var_idx(&variable.name) {
                variable.stack_idx.set(Some(idx));
            }
        }

        Ok(())
    }

    fn visit_assignment(&mut self, assignment: &expr::Assignment) -> EvalResult {
        self.evaluate_expr(&assignment.expr)?;

        let target_stack_idx_opt = {
            if let Some(local_environment) = &self.local_environments.last() {
                local_environment.get_var_idx(&assignment.target)
            } else {
                None
            }
        };

        if let Some(idx) = target_stack_idx_opt {
            assignment.stack_idx.set(Some(idx));
        }

        Ok(())
    }

    fn visit_call(&mut self, call: &expr::Call) -> EvalResult {
        self.evaluate_expr(&call.callee)?;
        for arg in &call.arguments {
            self.evaluate_expr(arg)?;
        }

        Ok(())
    }

    fn visit_nil(&self) -> EvalResult {
        return Ok(());
    }
}

