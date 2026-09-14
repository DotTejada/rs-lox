use crate::tokentype::TokenType;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Str(String),
    Identifier(String)
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize
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

    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.kind, self.lexeme, self.literal)
    }
}
