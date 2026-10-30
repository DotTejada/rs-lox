use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::loxfunction::LoxFunction;
use crate::{environment::Environment, error::RuntimeError, expr::ENode, expr::Expr, stmt::Stmt, token::Literal, tokentype::TokenType};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Fn(NativeFunction),
    LoxFn(LoxFunction),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                write!(f, "{}", n)
            },
            Value::Str(s) => {
                write!(f, "{}", s)
            },
            Value::Bool(b) => {
                write!(f, "{}", b)
            },
            Value::Fn(native_fn) => {
                write!(f, "{}", native_fn)
            },
            Value::LoxFn(lox_fn) => {
                write!(f, "{}", lox_fn)
            },
            Value::Nil => {
                write!(f, "nil")
            },
        }
    }
}

fn is_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(n1), Value::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
        (Value::Str(s1), Value::Str(s2)) => s1 == s2,
        (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
        (Value::Nil, Value::Nil) => true,
        (_, _) => false,
    }
    //return a.equals(b);
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Nil => false,
        Value::Bool(b) => *b,
        _ => true
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    arity: usize,
    call_fn: fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native fn>")
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, RuntimeError>;
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize { self.arity }
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, RuntimeError> {
        (self.call_fn)(interpreter, arguments)
    }
}

pub struct Interpreter {
    pub result: String,
    pub globals: Rc<RefCell<Environment>>,
    pub locals: HashMap<usize, usize>,
    pub environment: Rc<RefCell<Environment>>,
    pub ret_val: Option<Value>,
}

impl Interpreter {

    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define("clock".to_string(), Value::Fn(
            NativeFunction{
                arity: 0,
                call_fn: |_, _|
                    Ok(Value::Number(SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs() as f64))
            })
        );
        Self {
            result: String::new(),
            globals: globals.clone(),
            locals: HashMap::new(),
            environment: globals,
            ret_val: None,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in &statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn resolve(&mut self, expr: &ENode, depth: usize) {
        self.locals.insert(expr.id, depth);
    }

    fn evaluate(&mut self, expr: &ENode) -> Result<Value, RuntimeError> {
        self.visit(expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        self.visit_stmt(stmt)
    }

    pub fn execute_block(&mut self, statements: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        let result: Result<(), RuntimeError> = (|| {
            for stmt in statements {
                self.execute(stmt)?;
            }
            Ok(())
        })();
        self.environment = previous;
        result
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        if self.ret_val.is_some() {
            return Ok(());
        }
        match stmt {
            Stmt::Expression {expression} => {
                self.evaluate(&expression)?;
            },
            Stmt::Function(func) => {
                let function: LoxFunction = LoxFunction::new(func.clone(), self.environment.clone());
                self.environment.borrow_mut().define(func.name.lexeme.clone(), Value::LoxFn(function));
            },
            Stmt::If {condition, then_branch, else_branch} => {
                if is_truthy(&self.evaluate(&condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(eb) = else_branch {
                    self.execute(eb)?;
                }
            },
            Stmt::Print {expression} => {
                let value = self.evaluate(&expression)?;
                self.result.push_str(&format!("{}\n", value));
                print!("{}\n", value);
            },
            Stmt::Return {keyword: _, value} => {
                self.ret_val = if let Some(some_value) = value {
                    Some(self.evaluate(some_value)?)
                } else {
                    Some(Value::Nil)
                }
            },
            Stmt::Var {name, initializer} => {
                let value: Value = match initializer {
                    Some(init) => {
                        self.evaluate(&init)?
                    },
                    None => {
                        Value::Nil
                    }
                };
                self.environment.borrow_mut().define(name.lexeme.clone(), value);
            },
            Stmt::While {condition, body} => {
                while is_truthy(&self.evaluate(&condition)?) {
                    self.execute(body)?;
                }
            },
            Stmt::Block { statements } => {
                let environment = Rc::new(RefCell::new(
                    Environment::from(Rc::clone(&self.environment))
                ));
                self.execute_block(statements, environment)?;
            }
        }
        Ok(())
    }

    fn visit(&mut self, expr: &ENode) -> Result<Value, RuntimeError> {
        match &expr.expr {
            Expr::Binary{left, operator, right} => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                match (&l, operator.kind, &r) {
                    (Value::Number(lhs), TokenType::Greater, Value::Number(rhs)) => {
                        Ok(Value::Bool(lhs > rhs))
                    },
                    (Value::Number(lhs), TokenType::GreaterEqual, Value::Number(rhs)) => {
                        Ok(Value::Bool(lhs >= rhs))
                    },
                    (Value::Number(lhs), TokenType::Less, Value::Number(rhs)) => {
                        Ok(Value::Bool(lhs < rhs))
                    },
                    (Value::Number(lhs), TokenType::LessEqual, Value::Number(rhs)) => {
                        Ok(Value::Bool(lhs <= rhs))
                    },
                    (_, TokenType::BangEqual, _) => {
                        Ok(Value::Bool(!is_equal(&l, &r)))
                    },
                    (_, TokenType::EqualEqual, _) => {
                        Ok(Value::Bool(is_equal(&l, &r)))
                    },
                    (Value::Number(lhs), TokenType::Minus, Value::Number(rhs)) => {
                        Ok(Value::Number(lhs - rhs))
                    },
                    (Value::Number(lhs), TokenType::Plus, Value::Number(rhs)) => {
                        Ok(Value::Number(lhs + rhs))
                    },
                    (Value::Str(lhs), TokenType::Plus, Value::Str(rhs)) => {
                        Ok(Value::Str(format!("{}{}", lhs, rhs)))
                    },
                    (_, TokenType::Plus, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be two numbers or two strings.".to_string()})
                    },
                    (Value::Number(lhs), TokenType::Slash, Value::Number(rhs)) => {
                        Ok(Value::Number(lhs / rhs))
                    },
                    (Value::Number(lhs), TokenType::Star, Value::Number(rhs)) => {
                        Ok(Value::Number(lhs * rhs))
                    },
                    (_, TokenType::Greater, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::GreaterEqual, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Less, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::LessEqual, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Minus, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Slash, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Star, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operands must be numbers.".to_string()})
                    },
                    _ => {
                        Err(RuntimeError{token: operator.clone(), message: "Unreachable".to_string()})
                    }
                }
            },
            Expr::Call{callee, paren, arguments} => {
                let callee = self.evaluate(callee)?;
                let mut args: Vec<Value> = Vec::new();
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }
                let function = callee;
                match function {
                    Value::Fn(native_fn) => {
                        if arguments.len() != native_fn.arity() {
                            return Err(RuntimeError{
                                token: paren.clone(),
                                message: format!("Expected {} arguments but got {}.", native_fn.arity(), arguments.len())
                            })
                        }
                        native_fn.call(self, &args)
                    },
                    Value::LoxFn(lox_fn) => {
                        if arguments.len() != lox_fn.arity() {
                            return Err(RuntimeError{
                                token: paren.clone(),
                                message: format!("Expected {} arguments but got {}.", lox_fn.arity(), arguments.len())
                            })
                        }
                        lox_fn.call(self, &args)
                    },
                    _ => {
                        Err(RuntimeError{token: paren.clone(), message: "Can only call functions and classes.".to_string()})
                    }
                }
            },
            Expr::Grouping{expression} => {
                self.evaluate(expression)
            },
            Expr::Literal{value} => {
                match value {
                    Literal::Number(n) => {
                        Ok(Value::Number(*n))
                    },
                    Literal::Str(s) => {
                        Ok(Value::Str(s.clone()))
                    },
                    Literal::True => {
                        Ok(Value::Bool(true))
                    },
                    Literal::False => {
                        Ok(Value::Bool(false))
                    },
                    Literal::Nil => {
                        Ok(Value::Nil)
                    },
                }
            },
            Expr::Logical {left, operator, right} => {
                let left = self.evaluate(left)?;
                if operator.kind == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else {
                    if !is_truthy(&left) {
                        return Ok(left);
                    }
                }
                self.evaluate(right)
            },
            Expr::Unary{operator, right} => {
                let right = self.evaluate(right)?;
                match (operator.kind, &right) {
                    (TokenType::Minus, Value::Number(rhs)) => {
                        Ok(Value::Number(-rhs))
                    },
                    (TokenType::Bang, _) => {
                        Ok(Value::Bool(!is_truthy(&right)))
                    },
                    (TokenType::Minus, _) => {
                        Err(RuntimeError{token: operator.clone(), message: "Operand must be a number.".to_string()})
                    },
                    _ => {
                        Err(RuntimeError{token: operator.clone(), message: "Unreachable".to_string()})
                    }
                }
            },
            Expr::Variable{name} => {
                self.environment.borrow().get(name.clone())
            },
            Expr::Assign{name, value} => {
                let value = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name.clone(), value.clone())?;
                Ok(value)
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{parser::Parser, scanner::Scanner, token::Token};

    #[test]
    fn interpret_math() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_math =====");
        dbg!();
        let source_string = String::from("print 34.56 * 3 - 21 / 2 + 54.23 - 90;\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("57.41\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn interpret_eq() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_eq =====");
        dbg!();
        let source_string = String::from("print (42 + 5) == 47;\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("true\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn interpret_strconcat() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_strconcat =====");
        dbg!();
        let source_string = String::from("print \"hello\" + \" world\";\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("hello world\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn interpret_fib() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_fib =====");
        dbg!();
        let source_string = String::from(
"var a = 0;
var temp;
for (var b = 1; a < 10000; b = temp + b) {
    print a;
    temp = a;
    a = b;
}
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!(
"0
1
1
2
3
5
8
13
21
34
55
89
144
233
377
610
987
1597
2584
4181
6765
"
       , interpreter.result);
        Ok(())
    }

    #[test]
    fn interpret_while() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_while =====");
        dbg!();
        let source_string = String::from(
"{
    var i = 0;
    while (i < 10) {
        print i;
        i = i + 1;
    }
}
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!(
"0
1
2
3
4
5
6
7
8
9
"
            , interpreter.result);
        Ok(())
    }

    #[test]
    fn interpret_cstylefor() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_cstylefor =====");
        dbg!();
        let source_string = String::from("for (var i = 0; i < 10; i = i + 1) print i;");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!(
"0
1
2
3
4
5
6
7
8
9
"
            , interpreter.result);
        Ok(())
    }

    #[test]
    fn interpret_truthiness() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_cstylefor =====");
        dbg!();
        let source_string = String::from("print \"hi\" or 2;\nprint nil or \"yes\";\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("hi\nyes\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn function_add() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: function_add =====");
        dbg!();
        let source_string = String::from(
"fun add(a, b, c) {
    print a + b + c;
}
add(1, 2, 3);
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("6\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn function_add_print() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: function_add =====");
        dbg!();
        let source_string = String::from(
"fun add(a, b, c) {
    print a + b + c;
}
print add;
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("<fn add>\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn function_sayhi() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: function_sayhi =====");
        dbg!();
        let source_string = String::from(
"fun sayHi(first, last) {
    print \"Hi, \" + first + \" \" + last + \"!\";
}
sayHi(\"Dear\", \"Reader\");
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("Hi, Dear Reader!\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn function_fib() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: function_fib =====");
        dbg!();
        let source_string = String::from(
"fun fib(n) {
    if (n <= 1) return n;
    return fib(n - 2) + fib(n - 1);
}

for (var i = 0; i < 20; i = i + 1) {
    print fib(i);
}
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!(
"0
1
1
2
3
5
8
13
21
34
55
89
144
233
377
610
987
1597
2584
4181
"
            , interpreter.result);
        Ok(())
    }

    #[test]
    fn function_counter() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: function_counter =====");
        dbg!();
        let source_string = String::from(
"fun makeCounter() {
    var i = 0;
    fun count() {
        i = i + 1;
        print i;
    }

    return count;
}

var counter = makeCounter();
counter();
counter();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.interpret(expression)?;

        assert_eq!("1\n2\n", interpreter.result);
        Ok(())
    }

}
