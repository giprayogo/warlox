use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::token::{Token, Value};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(RuntimeError::UndefinedVariable(
                name.lexeme.clone(),
                name.line,
            )),
        }
    }
}
