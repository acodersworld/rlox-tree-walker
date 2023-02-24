use crate::eval_value::EvalValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, EvalValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<EvalValue> {
        self.values.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, value: EvalValue) {
        self.values.insert(name.to_string(), value);
    }
}
