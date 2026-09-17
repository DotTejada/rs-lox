use crate::{error::{ParseError, error_token}, expr::Expr, stmt::Stmt, token::{Literal, Token}, tokentype::TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
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

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Box::new(Expr::Variable{name: self.previous()}));
        }

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

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }
            match self.peek().kind {
                TokenType::Class => {},
                TokenType::Fun => {},
                TokenType::Var => {},
                TokenType::For => {},
                TokenType::If => {},
                TokenType::While => {},
                TokenType::Print => {},
                TokenType::Return => {},
                _ => { return; },
            }
            self.advance();
        }
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.".to_string())?;
        Ok(Stmt::Expression{expression: expr})
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print{expression: value})
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_stmt();
        }
        self.expr_stmt()
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;

        let mut initializer: Option<Box<Expr>> = None;
        if self.match_token(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string())?;
        Ok(Stmt::Var{name: name, initializer: initializer})
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_decl();
        }
        self.statement()
        //synchronize if got an error
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{expr::AstPrinter, scanner::Scanner};

    #[test]
    fn parse_unary() -> Result<(), Box<dyn std::error::Error>> {
        let source_string = String::from("!true == -97 + 38;");
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let statements = parser.parse()?;
        for stmt in statements {
            let expr = match stmt {
                Stmt::Expression {expression} => {
                    expression
                },
                _ => {
                    panic!("Statement is wrong kind");
                },
            };
            assert_eq!("(== (! true) (+ (- 97) 38))", AstPrinter{}.print(&*expr));
        }

        Ok(())
    }

    #[test]
    fn parse_math() -> Result<(), Box<dyn std::error::Error>> {
        let source_string = String::from("12 * 34 - 89 + 56 / 75 - 65 + 45 * 90;");
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let statements = parser.parse()?;
        for stmt in statements {
            let expr = match stmt {
                Stmt::Expression {expression} => {
                    expression
                },
                _ => {
                    panic!("Statement is wrong kind");
                },
            };
            assert_eq!("(+ (- (+ (- (* 12 34) 89) (/ 56 75)) 65) (* 45 90))", AstPrinter{}.print(&*expr));
        }

        Ok(())
    }

    #[test]
    fn parse_logic() -> Result<(), Box<dyn std::error::Error>> {
        let source_string = String::from("true != false == ( true == false );");
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let statements = parser.parse()?;
        for stmt in statements {
            let expr = match stmt {
                Stmt::Expression {expression} => {
                    expression
                },
                _ => {
                    panic!("Statement is wrong kind");
                },
            };
            assert_eq!("(== (!= true false) (group (== true false)))", AstPrinter{}.print(&*expr));
        }

        Ok(())
    }
}
