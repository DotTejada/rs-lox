use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::{Callable, Interpreter, Value}, loxfunction::LoxFunction, loxinstance::LoxInstance};

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> Rc<Self> {
        Rc::new(
            Self {
                name,
                methods
            }
        )
    }

    pub fn find_method(&self, name: &String) -> Option<LoxFunction> {
        self.methods.get(name).cloned()
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Callable for Rc<LoxClass> {
    fn arity(&self) -> usize {
        match self.find_method(&"init".to_string()) {
            Some(init) => {
                init.arity()
            },
            None => 0
        }
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, RuntimeError> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(Rc::clone(self))));
        let initializer = self.find_method(&"init".to_string());
        if let Some(init) = initializer {
            init.bind(instance.clone()).call(interpreter, arguments)?;
        };
        Ok(Value::LoxInstance(instance))
    }
}
