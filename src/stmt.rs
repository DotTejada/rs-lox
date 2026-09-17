use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Block {
        expression: Vec<Box<Stmt>>,
    },
    Class {
        name: Token,
        superclass: Box<Expr>,
        methods: Vec<Box<Stmt>>,
    },
    Expression {
        expression: Box<Expr>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print {
        expression: Box<Expr>,
    },
    Return {
        keyword: Token,
        value: Box<Expr>,
    },
    Variable {
        name: Token,
        initializer: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}
