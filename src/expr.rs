use crate::token::{Literal, Token};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    fn accept(&self) -> String {
        match self {
            Expr::Binary{left, operator, right} => {
                return parenthesize(&operator.lexeme, &[left, right]);
            },
            Expr::Grouping{expression} => {
                return parenthesize("group", &[expression]);
            },
            Expr::Literal{value} => {
                return format!("{}", value);
            },
            Expr::Unary{operator, right} => {
                return parenthesize(&operator.lexeme, &[right]);
            },
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
    return builder;
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        return expr.accept();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokentype::TokenType;

    #[test]
    fn ast_simple() {
        let expression: Box<Expr> = Box::new(Expr::Binary{
            left: Box::new(Expr::Literal{value: Literal::Number(1.0)}),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal{value: Literal::Number(2.0)}),
        });
        assert_eq!("(+ 1 2)", AstPrinter{}.print(&*expression))
    }

    #[test]
    fn ast_example() {
        let expression: Box<Expr> = Box::new(Expr::Binary{
            left: Box::new(Expr::Unary{
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal{value: Literal::Number(123.0)}),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping{expression: Box::new(Expr::Literal{value: Literal::Number(45.67)})}),
        });
        assert_eq!("(* (- 123) (group 45.67))", AstPrinter{}.print(&*expression))
    }

}
