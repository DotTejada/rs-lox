use std::collections::HashMap;

use crate::{error::{ScanError}, token::{Literal, Token}, tokentype::TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alphanum(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            keywords: vec![
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
            ].into_iter()
             .map(|(k, v)| (k.to_string(), v))
             .collect(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
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
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn string(&mut self) -> Result<(), ScanError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err( ScanError {what: "Unterminated string".to_string(), place: "".to_string(), line: self.line});
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        let value_str: String = value.iter().collect();
        self.add_token(TokenType::String, Some(Literal::Str(value_str)));
        Ok(())
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

    fn scan_token(&mut self) -> Result<(), ScanError> {
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
            '"' => { self.string()?; },
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err( ScanError {what: "Unexpected character".to_string(), place: "".to_string(), line: self.line} );
                }
            },
        }
        Ok(())
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        Ok(self.tokens)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn scan_math() -> Result<(), ScanError> {
        println!("===== test: scan_math =====");
        dbg!();
        let source_string = String::from("a + b; b - c; c / d; d * e;");
        let source: Vec<char> = source_string.chars().collect();
        let scanner: Scanner = Scanner::new(source);

        let scanner_output: Vec<Token> = scanner.scan_tokens()?;
        let expected_output: Vec<Token> = vec![
            Token::new( TokenType::Identifier, "a".to_string(), None, 1, ),
            Token::new( TokenType::Plus,       "+".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "b".to_string(), None, 1, ),
            Token::new( TokenType::Semicolon,  ";".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "b".to_string(), None, 1, ),
            Token::new( TokenType::Minus,      "-".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "c".to_string(), None, 1, ),
            Token::new( TokenType::Semicolon,  ";".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "c".to_string(), None, 1, ),
            Token::new( TokenType::Slash,      "/".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "d".to_string(), None, 1, ),
            Token::new( TokenType::Semicolon,  ";".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "d".to_string(), None, 1, ),
            Token::new( TokenType::Star,       "*".to_string(), None, 1, ),
            Token::new( TokenType::Identifier, "e".to_string(), None, 1, ),
            Token::new( TokenType::Semicolon,  ";".to_string(), None, 1, ),
            Token::new( TokenType::Eof,        "".to_string(),  None, 1, ),
        ];
        assert_eq!(*scanner_output, expected_output);
        println!("Input:\n{}\n=====\nOutput: {:#?}", source_string, expected_output);
        Ok(())
    }

    #[test]
    fn scan_comment() -> Result<(), ScanError> {
        println!("===== test: scan_comment =====");
        dbg!();
        let source_string = String::from("// this is a comment");
        let source: Vec<char> = source_string.chars().collect();
        let scanner: Scanner = Scanner::new(source);

        let scanner_output: Vec<Token> = scanner.scan_tokens()?;
        let expected_output: Vec<Token> = vec![
            Token::new( TokenType::Eof, "".to_string(), None, 1, ),
        ];
        assert_eq!(*scanner_output, expected_output);
        println!("Input:\n{}\n=====\nOutput: {:#?}", source_string, expected_output);
        Ok(())
    }

    #[test]
    fn scan_string() -> Result<(), ScanError> {
        println!("===== test: scan_string =====");
        dbg!();
        let source_string = String::from("var foo = \"hello\";");
        let source: Vec<char> = source_string.chars().collect();
        let scanner: Scanner = Scanner::new(source);

        let scanner_output: Vec<Token> = scanner.scan_tokens()?;
        let expected_output: Vec<Token> = vec![
            Token::new( TokenType::Var, "var".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "foo".to_string(), None, 1 ),
            Token::new( TokenType::Equal, "=".to_string(), None, 1 ),
            Token::new( TokenType::String, "\"hello\"".to_string(), Some(Literal::Str("hello".to_string())), 1 ),
            Token::new( TokenType::Semicolon, ";".to_string(), None, 1 ),
            Token::new( TokenType::Eof, "".to_string(), None, 1 ),
        ];
        assert_eq!(*scanner_output, expected_output);
        println!("Input:\n{}\n=====\nOutput: {:#?}", source_string, expected_output);
        Ok(())
    }

    #[test]
    fn scan_multichar_tokens() -> Result<(), ScanError> {
        println!("===== test: scan_multichar_tokens =====");
        dbg!();
        let source_string = String::from("a <= b; b >= c; c != d; d == e;");
        let source: Vec<char> = source_string.chars().collect();
        let scanner: Scanner = Scanner::new(source);

        let scanner_output: Vec<Token> = scanner.scan_tokens()?;
        let expected_output: Vec<Token> = vec![
            Token::new( TokenType::Identifier, "a".to_string(), None, 1 ),
            Token::new( TokenType::LessEqual, "<=".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "b".to_string(), None, 1 ),
            Token::new( TokenType::Semicolon, ";".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "b".to_string(), None, 1 ),
            Token::new( TokenType::GreaterEqual, ">=".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "c".to_string(), None, 1 ),
            Token::new( TokenType::Semicolon, ";".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "c".to_string(), None, 1 ),
            Token::new( TokenType::BangEqual, "!=".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "d".to_string(), None, 1 ),
            Token::new( TokenType::Semicolon, ";".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "d".to_string(), None, 1 ),
            Token::new( TokenType::EqualEqual, "==".to_string(), None, 1 ),
            Token::new( TokenType::Identifier, "e".to_string(), None, 1 ),
            Token::new( TokenType::Semicolon, ";".to_string(), None, 1 ),
            Token::new( TokenType::Eof, "".to_string(), None, 1 ),
        ];
        assert_eq!(*scanner_output, expected_output);
        println!("Input:\n{}\n=====\nOutput: {:#?}", source_string, expected_output);
        Ok(())
    }

    #[test]
    fn scan_loop() -> Result<(), ScanError> {
        println!("===== test: scan_loop =====");
        dbg!();
        let source_string = String::from("for (var a = 1; a < 10; a = a + 1) {\n  print a;\n}");
        let source: Vec<char> = source_string.chars().collect();
        let scanner: Scanner = Scanner::new(source);

        let scanner_output: Vec<Token> = scanner.scan_tokens()?;
        let expected_output: Vec<Token> = vec![
            Token::new( TokenType::For,        "for".to_string(),   None, 1 ),
            Token::new( TokenType::LeftParen,  "(".to_string(),     None, 1 ),
            Token::new( TokenType::Var,        "var".to_string(),   None, 1 ),
            Token::new( TokenType::Identifier, "a".to_string(),     None, 1 ),
            Token::new( TokenType::Equal,      "=".to_string(),     None, 1 ),
            Token::new( TokenType::Number,     "1".to_string(),     Some(Literal::Number(1.0)), 1 ),
            Token::new( TokenType::Semicolon,  ";".to_string(),     None, 1 ),
            Token::new( TokenType::Identifier, "a".to_string(),     None, 1 ),
            Token::new( TokenType::Less,       "<".to_string(),     None, 1 ),
            Token::new( TokenType::Number,     "10".to_string(),    Some(Literal::Number(10.0)), 1 ),
            Token::new( TokenType::Semicolon,  ";".to_string(),     None, 1 ),
            Token::new( TokenType::Identifier, "a".to_string(),     None, 1 ),
            Token::new( TokenType::Equal,      "=".to_string(),     None, 1 ),
            Token::new( TokenType::Identifier, "a".to_string(),     None, 1 ),
            Token::new( TokenType::Plus,       "+".to_string(),     None, 1 ),
            Token::new( TokenType::Number,     "1".to_string(),     Some(Literal::Number(1.0)), 1 ),
            Token::new( TokenType::RightParen, ")".to_string(),     None, 1 ),
            Token::new( TokenType::LeftBrace,  "{".to_string(),     None, 1 ),
            Token::new( TokenType::Print,      "print".to_string(), None, 2 ),
            Token::new( TokenType::Identifier, "a".to_string(),     None, 2 ),
            Token::new( TokenType::Semicolon,  ";".to_string(),     None, 2 ),
            Token::new( TokenType::RightBrace, "}".to_string(),     None, 3 ),
            Token::new( TokenType::Eof,        "".to_string(),      None, 3 ),
        ];
        assert_eq!(*scanner_output, expected_output);
        println!("Input:\n{}\n=====\nOutput: {:#?}", source_string, expected_output);
        Ok(())
    }
}
