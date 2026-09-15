use std::fmt;

use crate::tokentype::TokenType;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    Number(f64),
    Str(String),
    Identifier(String)
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(s) => {
                write!(f, "{}", s)
            },
            Literal::Number(n) => {
                write!(f, "{}", n)
            },
            Literal::Identifier(s) => {
                write!(f, "{}", s)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            kind: kind,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }

    //fn to_string(&self) -> String {
    //    format!("{:?} {} {:?}", self.kind, self.lexeme, self.literal)
    //}
}
