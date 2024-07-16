use std::cell::RefCell;
use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::token::{Token, Value};
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Value, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(v) => {
                // Challenge 8.2: don't allow use of uninitialized variable.
                if let Some(v) = v {
                    Ok(v.clone())
                } else {
                    Err(RuntimeError::UninitializedVariable(
                        token.lexeme.clone(),
                        token.line,
                    ))
                }
            }
            None => {
                if let Some(enclosing) = &self.enclosing {
                    enclosing.borrow().get(token)
                } else {
                    Err(RuntimeError::UndefinedVariable(
                        token.lexeme.clone(),
                        token.line,
                    ))
                }
            }
        }
    }

    // TODO: Consider consuming token?
    pub fn assign(&mut self, name: &Token, value: Option<Value>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            // * NOTE: No particular reasons for this. Can also do JS-style return value.
            Ok(())
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
