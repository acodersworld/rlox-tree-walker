use crate::environment::Environment;
use crate::interpreter::InterpreterContext;
use crate::stmt;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxFunction {
    pub declaration: Rc<stmt::Function>,
    pub closure: Option<Environment>
}

impl LoxFunction {
    pub fn call(
        &self,
        global_environment: &mut Environment,
        arguments: &Vec<EvalValue>,
    ) -> Result<EvalValue, String> {
        let mut environment = {
            match &self.closure {
                None => Environment::new(),
                Some(closure) => Environment::new_capture_env(&closure)
            }
        };

        let parameters = &self.declaration.parameters;
        for arg in parameters.iter().zip(arguments.iter()) {
            environment.set(arg.0, arg.1.clone());
        }

        let mut local_interpreter =
            InterpreterContext::new_with_local_env(global_environment, environment);
        if let Some(result) = local_interpreter.execute_many(&self.declaration.statements)? {
            return Ok(result);
        } else {
            return Ok(EvalValue::Nil);
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvalValue {
    Number(f32),
    Str(Rc<String>),
    Bool(bool),
    Function(Rc<LoxFunction>),
    Nil,
}

impl fmt::Display for EvalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::Number(n) => write!(f, "{}", n),
            EvalValue::Str(s) => write!(f, "{}", s),
            EvalValue::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            EvalValue::Function(func) => write!(f, "Lox function <{}>", func.declaration.name),
            EvalValue::Nil => write!(f, "nil"),
        }
    }
}
