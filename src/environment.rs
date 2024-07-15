use std::cell::RefCell;
use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::token::{Token, Value};
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    enclosing.borrow().get(name)
                } else {
                    Err(RuntimeError::UndefinedVariable(
                        name.lexeme.clone(),
                        name.line,
                    ))
                }
            }
        }
    }

    // TODO: Consider consuming token?
    pub fn assign(&mut self, name: &Token, value: Value) -> Result<Value, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            // * NOTE: No particular reasons for this. Can also do JS-style return value.
            Ok(Value::Null)
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(
                name.lexeme.clone(),
                name.line,
            ))
        }
    }
}
