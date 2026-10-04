use core::fmt;
use std::error::Error;

use crate::{token::Token, tokentype::TokenType};

#[derive(Debug)]
pub struct ScanError {
    pub what: String,
    pub place: String,
    pub line: usize,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error{}: {}", self.line, self.place, self.what)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.kind == TokenType::Eof {
            write!(f, "[line {}] Error at end: {}", self.token.line, self.message)
        } else {
            write!(f, "[line {}] Error at '{}': {}", self.token.line, self.token.lexeme, self.message)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.token.line)
    }
}

impl Error for ScanError{}
impl Error for ParseError{}
impl Error for RuntimeError{}

pub fn report(line: usize, place: &str, what: &str) {
    println!("[line {}] Error{}: {}", line, place, what);
    //had_error = true;
}

pub fn error(err: ScanError) {
    println!("[line {}] Error: {}", err.line, err.what);
}

pub fn error_token(token: Token, message: &str) {
    if token.kind == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), &message);
    }
}
