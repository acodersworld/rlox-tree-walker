use crate::eval_value::EvalValue;
use std::cell::{RefCell, RefMut};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
struct StackValue {
    hash: u64,
    value: Rc<RefCell<EvalValue>>,
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: Vec<StackValue>,
    scope_stack: Vec<usize>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: vec![],
            scope_stack: vec![],
        }
    }

    pub fn new_capture_env(enclosing: &Environment) -> Environment {
        Environment {
            values: enclosing.values.clone(),
            scope_stack: vec![enclosing.values.len()],
        }
    }

    pub fn define_var(&mut self, name: &str, value: EvalValue) {
        let name_hash = Environment::hash_name(name);
        let bottom = *self.scope_stack.last().unwrap_or(&0);

        for stack_value in self.values[bottom..].iter_mut().rev() {
            if stack_value.hash == name_hash {
                stack_value.value = Rc::new(RefCell::new(value));
                return;
            }
        }

        self.values.push(StackValue {
            hash: Environment::hash_name(name),
            value: Rc::new(RefCell::new(value)),
        });
    }

    pub fn get_var(&self, name: &str) -> Option<EvalValue> {
        self.find_eval_value(name, 0)
            .map(|ref_mut_value| ref_mut_value.clone())
    }

    pub fn set_var(&mut self, name: &str, value: EvalValue) {
        {
            if let Some(mut mut_ref_eval_value) = self.find_eval_value(name, 0) {
                *mut_ref_eval_value = value;
                return;
            }
        }

        self.values.push(StackValue {
            hash: Environment::hash_name(name),
            value: Rc::new(RefCell::new(value)),
        });
    }

    pub fn push_scope(&mut self) {
        self.scope_stack.push(self.values.len());
    }

    pub fn pop_scope(&mut self) {
        self.values.truncate(*self.scope_stack.last().unwrap_or(&0));
    }

    fn hash_name(name: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish()
    }

    fn find_eval_value(&self, name: &str, stack_bottom_idx: usize) -> Option<RefMut<EvalValue>> {
        let name_hash = Environment::hash_name(name);

        for stack_value in self.values[stack_bottom_idx..].iter().rev() {
            if stack_value.hash == name_hash {
                return Some(stack_value.value.borrow_mut());
            }
        }

        None
    }
}
