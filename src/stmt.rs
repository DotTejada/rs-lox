use crate::{expr::ENode, token::Token};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        superclass: Option<ENode>,
        methods: Vec<Function>,
    },
    Expression {
        expression: ENode,
    },
    Function(Function),
    If {
        condition: ENode,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: ENode,
    },
    Return {
        keyword: Token,
        value: Option<ENode>,
    },
    Var {
        name: Token,
        initializer: Option<ENode>,
    },
    While {
        condition: ENode,
        body: Box<Stmt>,
    },
}
