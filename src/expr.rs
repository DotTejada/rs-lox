use std::sync::atomic::{AtomicUsize, Ordering};

use crate::token::{Literal, Token};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct ENode {
    pub id: usize,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<ENode>,
    },
    Binary {
        left: Box<ENode>,
        operator: Token,
        right: Box<ENode>,
    },
    Call {
        callee: Box<ENode>,
        paren: Token,
        arguments: Vec<ENode>,
    },
    //Get {
    //    object: Box<Expr>,
    //    name: Token,
    //},
    Grouping {
        expression: Box<ENode>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<ENode>,
        operator: Token,
        right: Box<ENode>,
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
        right: Box<ENode>,
    },
    Variable {
        name: Token,
    },
}

//impl Hash for ENode {
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        self.id.hash(state);
//    }
//}
//
//impl PartialEq for ENode {
//    fn eq(&self, other: &Self) -> bool {
//        self.id == other.id
//    }
//}
//
//impl Eq for ENode {}

impl ENode {
    pub fn new(expr: Expr) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            expr,
        }
    }
}

impl ENode {
    fn accept(&self) -> String {
        match &self.expr {
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

fn parenthesize(name: &str, exprs: &[&Box<ENode>]) -> String {
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
    pub fn print(&self, expr: &ENode) -> String {
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
        let expression: Box<ENode> = Box::new(ENode::new(Expr::Binary{
            left: Box::new(ENode::new(Expr::Literal{value: Literal::Number(1.0)})),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Box::new(ENode::new(Expr::Literal{value: Literal::Number(2.0)})),
        }));
        let expected = "(+ 1 2)";
        println!("Input:\n{:#?}\n=====\nOutput: {}", expression, expected);
        assert_eq!(expected, AstPrinter{}.print(&*expression))
    }

    #[test]
    fn ast_example() {
        println!("===== test: ast_example =====");
        dbg!();
        let expression: Box<ENode> = Box::new(ENode::new(Expr::Binary{
            left: Box::new(ENode::new(Expr::Unary{
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(ENode::new(Expr::Literal{value: Literal::Number(123.0)})),
            })),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(ENode::new(Expr::Grouping{expression: Box::new(ENode::new(Expr::Literal{value: Literal::Number(45.67)}))})),
        }));
        let expected = "(* (- 123) (group 45.67))";
        println!("Input:\n{:#?}\n=====\nOutput: {}", expression, expected);
        assert_eq!(expected, AstPrinter{}.print(&*expression))
    }
}
