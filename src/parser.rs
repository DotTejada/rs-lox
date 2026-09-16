use crate::{error::{ParseError, error_token}, expr::Expr, token::{Literal, Token}, tokentype::TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&mut self, kind: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().kind == *kind;
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().kind == TokenType::Eof;
    }

    fn peek(&mut self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParseError> {
        return Ok(self.equality()?);
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary{left: expr, operator: operator, right: right});
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.term()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Expr::Binary{left: expr, operator: operator, right: right});
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary{left: expr, operator: operator, right: right});
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary{left: expr, operator: operator, right: right});
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary{operator: operator, right: right}));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.match_token(&[TokenType::False]) { return Ok(Box::new(Expr::Literal{value: Literal::False})); }
        if self.match_token(&[TokenType::True]) { return Ok(Box::new(Expr::Literal{value: Literal::True})); }
        if self.match_token(&[TokenType::Nil]) { return Ok(Box::new(Expr::Literal{value: Literal::Nil})); }

        if self.match_token(&[TokenType::Number, TokenType::String]) { return Ok(Box::new(Expr::Literal{value: self.previous().literal.unwrap() })); }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression".to_string())?;
            return Ok(Box::new(Expr::Grouping{expression: expr}));
        };
        return Err( ParseError{token: self.peek(), message: "Expect expression".to_string()} );
    }

    fn consume(&mut self, kind: TokenType, message: String) -> Result<Token, ParseError> {
        if self.check(&kind) {
            return Ok(self.advance());
        }
        error_token(self.peek(), &message);
        return Err( ParseError{token: self.peek(), message: message} );
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParseError> {
        return Ok(self.expression()?);
    }

}
