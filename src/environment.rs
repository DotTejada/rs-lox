use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::{error::RuntimeError, interpreter::Value, token::Token};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new()
        }
    }

    pub fn from(environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(environment),
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(r) => {
                Ok(r.clone())
            },
            None => {
                if let Some(e) = &self.enclosing {
                    return e.borrow().get(name);
                }
                Err(RuntimeError{token: name.clone(), message: format!("Undefined variable '{}'", &name.lexeme)})
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), RuntimeError> {
        match self.values.get_mut(&name.lexeme) {
            Some(existing_value) => {
                *existing_value = value;
                Ok(())
            },
            None => {
                if let Some(e) = self.enclosing.as_mut() {
                    return e.borrow_mut().assign(name, value);
                }
                Err(RuntimeError{token: name.clone(), message: format!("Undefined variable '{}'", &name.lexeme)})
            },
        }
    }
}
