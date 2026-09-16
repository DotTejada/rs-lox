use crate::{expr::Expr, token::Literal, tokentype::TokenType};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

pub struct Interpreter {
}


impl Interpreter {

    fn evaluate(&self, expr: Box<Expr>) -> Option<Value> {
        self.visit(expr)
    }

    fn is_truthy(&self, opt_val: Option<Value>) -> bool {
        match opt_val {
            Some(val) => {
                if let Value::Bool(b) = val {
                    return b;
                }
                return true;
            },
            None => {
                return false;
            }
        }
    }

    fn visit(&self, expr: Box<Expr>) -> Option<Value> {
        match *expr {
            Expr::Binary{left, operator, right} => {
                let lhs = self.evaluate(left)?;
                let rhs = self.evaluate(right)?;
                match operator.kind {
                    TokenType::Minus => {
                        return lhs - rhs;
                    },
                    TokenType::Slash => {
                        return lhs / rhs;
                    },
                    TokenType::Star => {
                        return lhs * rhs;
                    },
                    _ => {
                        None
                    }
                }
            },
            Expr::Grouping{expression} => {
                return self.evaluate(expression);
            },
            Expr::Literal{value} => {
                match value {
                    Literal::Number(n) => {
                        Some(Value::Number(*n))
                    },
                    Literal::Str(s) => {
                        Some(Value::Str(*s))
                    },
                    Literal::True => {
                        Some(Value::Bool(true))
                    },
                    Literal::False => {
                        Some(Value::Bool(false))
                    },
                    Literal::Nil => {
                        Some(Value::Nil)
                    },
                }
            },
            Expr::Unary{operator, right} => {
                let right = self.evaluate(right);
                match operator.kind {
                    TokenType::Minus => {
                        if let Some(Value::Number(n)) = right {
                            return Some(Value::Number(-n));
                        }
                    },
                    TokenType::Bang => {
                        return Some(Value::Bool(!self.is_truthy(right)));
                    },
                    _ => {}
                }
                //panic!("Unreachable");
                None
            },
        }
    }
}
