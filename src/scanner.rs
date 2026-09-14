use std::collections::HashMap;

use crate::{token::{Literal, Token}, tokentype::TokenType};

pub struct Scanner<'a> {
    source: Vec<char>,
    tokens: Vec<Token>,
    keywords: HashMap<&'a str, TokenType>,
    start: usize,
    current: usize,
    line: usize
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

fn is_alphanum(c: char) -> bool {
    return is_digit(c) || is_alpha(c);
}

impl Scanner<'_> {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            keywords: HashMap::from([
                ("and",    TokenType::And),
                ("class",  TokenType::Class),
                ("else",   TokenType::Else),
                ("false",  TokenType::False),
                ("for",    TokenType::For),
                ("fun",    TokenType::Fun),
                ("if",     TokenType::If),
                ("nil",    TokenType::Nil),
                ("or",     TokenType::Or),
                ("print",  TokenType::Print),
                ("return", TokenType::Return),
                ("super",  TokenType::Super),
                ("this",   TokenType::This),
                ("true",   TokenType::True),
                ("var",    TokenType::Var),
                ("while",  TokenType::While),
            ]),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, kind: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        let lexeme: String = text.iter().collect();
        if let Some(_) = literal {
            self.tokens.push(Token::new(kind, lexeme, literal, self.line));
        } else {
            self.tokens.push(Token::new(kind, lexeme, None, self.line));
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected { return false; }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1];
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            panic!("Unterminated string");
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        let value_str: String = value.iter().collect();
        self.add_token(TokenType::String, Some(Literal::Str(value_str)));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        let value_str: String = value.iter().collect();
        let f_val: f64 = value_str.parse().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Number(f_val)));
    }

    fn identifier(&mut self) {
        while is_alphanum(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let text_str: String = text.iter().collect();
        match self.keywords.get(text_str.as_str()) {
            Some(kind) => {
                self.add_token(*kind, None);
            },
            None => { 
                self.add_token(TokenType::Identifier, None);
            },
        }
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => { self.add_token(TokenType::LeftParen, None); },
            ')' => { self.add_token(TokenType::RightParen, None); },
            '{' => { self.add_token(TokenType::LeftBrace, None); },
            '}' => { self.add_token(TokenType::RightBrace, None); },
            ',' => { self.add_token(TokenType::Comma, None); },
            '.' => { self.add_token(TokenType::Dot, None); },
            '-' => { self.add_token(TokenType::Minus, None); },
            '+' => { self.add_token(TokenType::Plus, None); },
            ';' => { self.add_token(TokenType::Semicolon, None); },
            '*' => { self.add_token(TokenType::Star, None); },
            '!' => { 
                let kind = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(kind, None); 
            },
            '=' => { 
                let kind = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(kind, None); 
            },
            '<' => { 
                let kind = if self.match_char('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(kind, None); 
            },
            '>' => { 
                let kind = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(kind, None); 
            },
            '/' => { 
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None); 
                }
            },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => { self.line += 1; },
            '"' => { self.string(); },
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    panic!("Unexpected character");
                }
            },
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return &self.tokens;
    }
}
