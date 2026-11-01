use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::Value, loxclass::LoxClass, token::Token};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub klass: Rc<LoxClass>,
    fields: HashMap<String, Value>
}

impl LoxInstance {
    pub fn new(klass: Rc<LoxClass>) -> Self {
        Self {
            klass,
            fields: HashMap::new()
        }
    }

    pub fn get(instance: &Rc<RefCell<LoxInstance>>, name: &Token) -> Result<Value, RuntimeError> {
        let field = instance.borrow().fields.get(&name.lexeme).cloned();
        if let Some(v) = field {
            return Ok(v);
        }

        let method = instance.borrow().klass.find_method(&name.lexeme);
        if let Some(method) = method {
            return Ok(Value::LoxFn(method.bind(Rc::clone(instance))));
        }
        return Err(RuntimeError{
            token: name.clone(),
            message: format!("Undefined property '{}'.", name.lexeme)
        })
    }

    pub fn set(&mut self, name: &Token, value: &Value) {
        self.fields.insert(name.lexeme.clone(), value.clone());
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.klass)
    }
}
