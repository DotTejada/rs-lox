use std::collections::HashMap;

use crate::{error::RuntimeError, interpreter::Value, token::Token};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(r) => {
                Ok(r.clone())
            },
            None => {
                Err(RuntimeError{token: name.clone(), message: format!("Undefined variable '{}'", &name.lexeme)})
            },
        }
    }
}
