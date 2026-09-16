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
