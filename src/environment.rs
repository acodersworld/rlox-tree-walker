use crate::eval_value::EvalValue;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<EvalValue>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn new_capture_env(enclosing: &Environment) -> Environment {
        Environment {
            values: enclosing.values.clone()
        }
    }

    pub fn get(&self, name: &str) -> Option<EvalValue> {
        let val = self.values.get(name);
        match val {
            None => None,
            Some(rc_cell_eval_val) => Some(rc_cell_eval_val.borrow().clone())
        }
    }

    pub fn set(&mut self, name: &str, value: EvalValue) {
        let val = self.values.get(name);
        match val {
            None => {
                self.values.insert(name.to_string(), Rc::new(RefCell::new(value)));
            }
            Some(rc_cell_eval_val) => {
                *rc_cell_eval_val.borrow_mut() = value;
            }
        }
    }
}
