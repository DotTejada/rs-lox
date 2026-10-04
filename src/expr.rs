use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    //Call {
    //    callee: Box<Expr>,
    //    paren: Token,
    //    arguments: Vec<Box<Expr>>,
    //},
    //Get {
    //    object: Box<Expr>,
    //    name: Token,
    //},
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    //Set {
    //    object: Box<Expr>,
    //    name: Token,
    //    value: Box<Expr>,
    //},
    //Super {
    //    keyword: Token,
    //    method: Token,
    //},
    //This {
    //    keyword: Token,
    //},
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    fn accept(&self) -> String {
        match self {
            Expr::Binary{left, operator, right} => {
                parenthesize(&operator.lexeme, &[left, right])
            },
            Expr::Grouping{expression} => {
                parenthesize("group", &[expression])
            },
            Expr::Literal{value} => {
                format!("{}", value)
            },
            Expr::Unary{operator, right} => {
                parenthesize(&operator.lexeme, &[right])
            },
            _ => "Not implemented".to_string()
        }
    }
}

fn parenthesize(name: &str, exprs: &[&Box<Expr>]) -> String {
    let mut builder = String::new();
    builder.push('(');
    builder.push_str(name);
    for expr in exprs {
        builder.push(' ');
        builder.push_str(&expr.accept());
    }
    builder.push(')');
    builder
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokentype::TokenType;

    #[test]
    fn ast_simple() {
        println!("===== test: ast_simple =====");
        dbg!();
        let expression: Box<Expr> = Box::new(Expr::Binary{
            left: Box::new(Expr::Literal{value: Literal::Number(1.0)}),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal{value: Literal::Number(2.0)}),
        });
        let expected = "(+ 1 2)";
        println!("Input:\n{:#?}\n=====\nOutput: {}", expression, expected);
        assert_eq!(expected, AstPrinter{}.print(&*expression))
    }

    #[test]
    fn ast_example() {
        println!("===== test: ast_example =====");
        dbg!();
        let expression: Box<Expr> = Box::new(Expr::Binary{
            left: Box::new(Expr::Unary{
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal{value: Literal::Number(123.0)}),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping{expression: Box::new(Expr::Literal{value: Literal::Number(45.67)})}),
        });
        let expected = "(* (- 123) (group 45.67))";
        println!("Input:\n{:#?}\n=====\nOutput: {}", expression, expected);
        assert_eq!(expected, AstPrinter{}.print(&*expression))
    }
}