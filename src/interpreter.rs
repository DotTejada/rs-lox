use core::fmt;

use crate::{error::RuntimeError, expr::Expr, token::Literal, tokentype::TokenType};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Value {
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
            Value::Nil => {
                write!(f, "nil")
            },
        }
    }
}

pub struct Interpreter {}

impl Interpreter {

    pub fn interpret(&self, expression: Box<Expr>) -> Result<String, RuntimeError> {
        let value = self.evaluate(expression)?;
        Ok(format!("{}", value))
    }

    fn evaluate(&self, expr: Box<Expr>) -> Result<Value, RuntimeError> {
        self.visit(expr)
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
        a == b
        //return a.equals(b);
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Bool(b) => b,
            _ => true
        }
    }

    fn visit(&self, expr: Box<Expr>) -> Result<Value, RuntimeError> {
        match *expr {
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
                        Ok(Value::Bool(!self.is_equal(l, r)))
                    },
                    (_, TokenType::EqualEqual, _) => {
                        Ok(Value::Bool(self.is_equal(l, r)))
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
                        Err(RuntimeError{token: operator, message: "Operands must be two numbers or two strings.".to_string()})
                    },
                    (Value::Number(lhs), TokenType::Slash, Value::Number(rhs)) => {
                        Ok(Value::Number(lhs / rhs))
                    },
                    (Value::Number(lhs), TokenType::Star, Value::Number(rhs)) => {
                        Ok(Value::Number(lhs * rhs))
                    },
                    (_, TokenType::Greater, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::GreaterEqual, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Less, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::LessEqual, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Minus, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Slash, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    (_, TokenType::Star, _) => {
                        Err(RuntimeError{token: operator, message: "Operands must be numbers.".to_string()})
                    },
                    _ => {
                        Err(RuntimeError{token: operator, message: "Unreachable".to_string()})
                    }
                }
            },
            Expr::Grouping{expression} => {
                self.evaluate(expression)
            },
            Expr::Literal{value} => {
                match value {
                    Literal::Number(n) => {
                        Ok(Value::Number(n))
                    },
                    Literal::Str(s) => {
                        Ok(Value::Str(s))
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
            Expr::Unary{operator, right} => {
                let right = self.evaluate(right)?;
                match (operator.kind, &right) {
                    (TokenType::Minus, Value::Number(rhs)) => {
                        Ok(Value::Number(-rhs))
                    },
                    (TokenType::Bang, _) => {
                        Ok(Value::Bool(!self.is_truthy(right)))
                    },
                    (TokenType::Minus, _) => {
                        Err(RuntimeError{token: operator, message: "Operand must be a number.".to_string()})
                    },
                    _ => {
                        Err(RuntimeError{token: operator, message: "Unreachable".to_string()})
                    }
                }
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
        let source_string = String::from("34.56 * 3 - 21 / 2 + 54.23 - 90;\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let interpreter: Interpreter = Interpreter{};

        assert_eq!("57.41", interpreter.interpret(expression)?);
        Ok(())
    }

    #[test]
    fn interpret_eq() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_eq =====");
        dbg!();
        let source_string = String::from("(42 + 5) == 47;\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let interpreter: Interpreter = Interpreter{};

        assert_eq!("true", interpreter.interpret(expression)?);
        Ok(())
    }

    #[test]
    fn interpret_strconcat() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: interpret_strconcat =====");
        dbg!();
        let source_string = String::from("\"hello\" + \" world\";\n");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let interpreter: Interpreter = Interpreter{};

        assert_eq!("hello world", interpreter.interpret(expression)?);
        Ok(())
    }
}
