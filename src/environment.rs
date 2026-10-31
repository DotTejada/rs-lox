use crate::{error::RuntimeError, interpreter::Value, token::Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
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

    //pub fn ancestor(&mut self, distance: usize) -> Rc<RefCell<Environment>> {
    //    for i in 0..distance {
    //        self = self.enclosing;
    //    }
    //    return self
    //}

    pub fn get_at(&self, distance: usize, name: &String) -> Result<Value, RuntimeError> {
        //ancestor(distance).values.get(name)
        if distance == 0 {
            return Ok(self.values.get(name).unwrap().clone());
        } else {
            return self.enclosing.as_ref().unwrap().borrow().get_at(distance - 1, name);
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
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

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<(), RuntimeError> {
        match self.values.get_mut(&name.lexeme) {
            Some(existing_value) => {
                *existing_value = value.clone();
                Ok(())
            },
            None => {
                if let Some(e) = self.enclosing.as_mut() {
                    return e.borrow_mut().assign(name, value);
                }
                Err(RuntimeError { token: name.clone(), message: format!("Undefined variable '{}'", &name.lexeme) })
            },
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Value) -> Result<(), RuntimeError> {
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value.clone());
        } else {
            return self.enclosing.as_ref().unwrap().borrow_mut().assign_at(distance - 1, name, value);
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::resolver::Resolver;
    use crate::{parser::Parser, scanner::Scanner, token::Token};

    #[test]
    fn env_shadowing() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: env_shadowing =====");
        dbg!();
        let source_string = String::from(
"var a = \"global a\";
var b = \"global b\";
var c = \"global c\";
{
  var a = \"outer a\";
  var b = \"outer b\";
  {
    var a = \"inner a\";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!(
"inner a
outer b
global c
outer a
outer b
global c
global a
global b
global c
"
        , interpreter.result);
        Ok(())
    }

    #[test]
    fn env_simple_enclosing() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: env_simple_enclosing =====");
        dbg!();
        let source_string = String::from(
"var global = \"outside\";
{
    var local = \"inside\";
    print global + local;
}
"
        );
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("outsideinside\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn env_first_second() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: env_first_second =====");
        dbg!();
        let source_string = String::from(
"{
    var a = \"first\";
    print a;
}
{
    var b = \"second\";
    print b;
}
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("first\nsecond\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn env_simple() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: env_simple =====");
        dbg!();
        let source_string = String::from(
"var a = 1;
print a;
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("1\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn env_redefinition() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: env_redefinition =====");
        dbg!();
        let source_string = String::from(
            "var a = 1;
a = 2;
print a;
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("2\n", interpreter.result);
        Ok(())
    }
}
