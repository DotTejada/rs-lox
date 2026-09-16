use rs_lox::{token::{Literal, Token}, tokentype::TokenType};

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


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_ast() {
        let expression: Box<Expr> = Box::new(Expr::Binary{
            left: Box::new(Expr::Literal{value: Some(Literal::Number(1.0))}),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal{value: Some(Literal::Number(2.0))}),
        });
        assert_eq!("(+ 1 2)", AstPrinter{}.print(&*expression))
    }

    #[test]
    fn test_example_ast() {
        let expression: Box<Expr> = Box::new(Expr::Binary{
            left: Box::new(Expr::Unary{
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal{value: Some(Literal::Number(123.0))}),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping{expression: Box::new(Expr::Literal{value: Some(Literal::Number(45.67))})}),
        });
        assert_eq!("(* (- 123) (group 45.67))", AstPrinter{}.print(&*expression))
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
    println!("{}", AstPrinter{}.print(&*expression));
}
