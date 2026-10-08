use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, error::RuntimeError, interpreter::{Callable, Interpreter, Value}, stmt::Stmt, token::Token};

pub struct LoxFunction {
    declaration: Stmt
}

impl LoxFunction {
    pub fn new(declaration: Stmt) -> Self {
        Self {
            declaration
        }
    }
}

impl Callable for LoxFunction {

    fn arity(&self) -> usize {
        if let Stmt::Function{name: _, params, body: _} = &self.declaration {
            return params.len();
        }
        return 0;
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, RuntimeError> {
        let env = Rc::new(RefCell::new(Environment::from(interpreter.globals.clone())));
        if let Stmt::Function{name: _, params, body} = &self.declaration {
            for i in 0..params.len() {
                env.borrow_mut().define(params[i].lexeme.clone(), arguments[i].clone());
            }
            interpreter.execute_block(&body, env);
            return Ok(Value::Nil)
        } else {
            return Err(RuntimeError{token: name.clone(), message: "Unreachable".to_string()})
        }
    }
}
