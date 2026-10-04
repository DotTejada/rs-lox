use crate::{error::{ParseError, error_token}, expr::Expr, stmt::Stmt, token::{Literal, Token}, tokentype::TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&mut self, kind: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == *kind
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        Ok(self.assignment()?)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            return match expr {
                Expr::Variable{name} => {
                    Ok(Expr::Assign{name, value: Box::new(value) })
                },
                _ => {
                    Err(ParseError{token: equals, message: "Invalid assignment target.".to_string()})
                }
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical{left: Box::new(expr), operator, right: Box::new(right)};
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_token(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical{left: Box::new(expr), operator, right: Box::new(right)};
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary{left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary{left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary{left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary{left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary{operator, right: Box::new(right) });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) { return Ok(Expr::Literal{value: Literal::False}); }
        if self.match_token(&[TokenType::True]) { return Ok(Expr::Literal{value: Literal::True}); }
        if self.match_token(&[TokenType::Nil]) { return Ok(Expr::Literal{value: Literal::Nil}); }

        if self.match_token(&[TokenType::Number, TokenType::String]) { return Ok(Expr::Literal{value: self.previous().literal.unwrap() }); }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable{name: self.previous()});
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression".to_string())?;
            return Ok(Expr::Grouping{expression: Box::new(expr)});
        };
        Err( ParseError{token: self.peek(), message: "Expect expression".to_string()} )
    }

    fn consume(&mut self, kind: TokenType, message: String) -> Result<Token, ParseError> {
        if self.check(&kind) {
            return Ok(self.advance());
        }
        error_token(self.peek(), &message);
        Err( ParseError{token: self.peek(), message } )
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

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.".to_string())?;
        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.match_token(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::If{condition, then_branch: Box::new(then_branch), else_branch})
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.".to_string())?;
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::For]) {
            return self.for_stmt();
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_stmt();
        }
        if self.match_token(&[TokenType::Print]) {
            return self.print_stmt();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_stmt();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block{statements: self.block()?});
        }
        self.expr_stmt()
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.".to_string())?;
        let initializer: Option<Stmt>;
        if self.match_token(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(&[TokenType::Var]) {
            initializer = Some(self.var_decl()?);
        } else {
            initializer = Some(self.expr_stmt()?);
        }
        let mut condition = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.".to_string())?;
        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.".to_string())?;
        let mut body: Stmt = self.statement()?;
        if let Some(inc) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression {expression: inc}]
            }
        }
        if let None = condition {
            condition = Some(Expr::Literal {value: Literal::True});
        }
        body = Stmt::While {condition: condition.unwrap(), body: Box::new(body)};
        if let Some(init) = initializer {
            body = Stmt::Block {statements: vec![init, body]};
        }
        Ok(body)
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;

        let mut initializer: Option<Expr> = None;
        if self.match_token(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string())?;
        Ok(Stmt::Var{name, initializer })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after  condition.".to_string())?;
        let body = self.statement()?;
        Ok(Stmt::While{condition, body: Box::new(body)})
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
        println!("===== test: parse_unary =====");
        dbg!();
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
            let expected = "(== (! true) (+ (- 97) 38))";
            println!("Input:\n{}\n=====\nOutput: {}", source_string, expected);
            assert_eq!(expected, AstPrinter{}.print(&expr));
        }

        Ok(())
    }

    #[test]
    fn parse_math() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: parse_math =====");
        dbg!();
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
            let expected = "(+ (- (+ (- (* 12 34) 89) (/ 56 75)) 65) (* 45 90))";
            println!("Input:\n{}\n=====\nOutput: {}", source_string, expected);
            assert_eq!(expected, AstPrinter{}.print(&expr));
        }

        Ok(())
    }

    #[test]
    fn parse_logic() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: parse_logic =====");
        dbg!();
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
            let expected = "(== (!= true false) (group (== true false)))";
            println!("Input:\n{}\n=====\nOutput: {}", source_string, expected);
            assert_eq!(expected, AstPrinter{}.print(&expr));
        }

        Ok(())
    }
}
