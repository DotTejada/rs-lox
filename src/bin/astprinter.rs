use rs_lox::{token::{Literal, Token}, tokentype::TokenType};

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
        value: Option<Literal>,
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
                match value {
                    Some(v) => {
                        return format!("{}", *v);
                    },
                    None => {
                        return String::from("nil");
                    }
                }
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

fn main() {
    let expression: Box<Expr> = Box::new(Expr::Binary{
        left: Box::new(Expr::Unary{
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal{value: Some(Literal::Number(123.0))}),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping{expression: Box::new(Expr::Literal{value: Some(Literal::Number(45.67))})}),
    });
    let astprinter = AstPrinter{};
    println!("{}", astprinter.print(&*expression));
}
