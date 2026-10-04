use crate::tokentype::TokenType;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    Number(f64),
    Str(String),
    Identifier(String)
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
            kind,
            lexeme,
            literal,
            line,
        }
    }

    //fn to_string(&self) -> String {
    //    format!("{:?} {} {:?}", self.kind, self.lexeme, self.literal)
    //}
}
