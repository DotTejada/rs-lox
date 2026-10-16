use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, error::RuntimeError, interpreter::{Callable, Interpreter, Value}, stmt::Function};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Function,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl Callable for LoxFunction {

    fn arity(&self) -> usize {
        return self.declaration.params.len();
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, RuntimeError> {
        let env = Rc::new(RefCell::new(Environment::from(self.closure.clone())));
        for i in 0..self.declaration.params.len() {
            env.borrow_mut().define(self.declaration.params[i].lexeme.clone(), arguments[i].clone());
        }
        interpreter.execute_block(&self.declaration.body, env)?;
        if let Some(ret_val) = &interpreter.ret_val {
            let return_value = ret_val.clone();
            interpreter.ret_val = None;
            return Ok(return_value);
        }
        return Ok(Value::Nil)
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {} >", self.declaration.name.lexeme)
    }
}
