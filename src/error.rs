use crate::{token::Token, tokentype::TokenType};

#[derive(Debug)]
pub struct Error {
    pub what: String,
    pub place: String,
    pub line: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub fn report(line: usize, place: &str, what: &str) {
    println!("[line {}] Error{}: {}", line, place, what);
    //had_error = true;
}

pub fn error(err: Error) {
    println!("[line {}] Error: {}", err.line, err.what);
}

pub fn error_token(token: Token, message: &str) {
    if token.kind == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), &message);
    }
}
