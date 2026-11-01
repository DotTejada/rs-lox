use std::collections::HashMap;

use crate::{error::error_token, expr::ENode, expr::Expr, interpreter::Interpreter, stmt::{Function, Stmt}, token::Token};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    pub had_error: bool,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {

    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            had_error: false,
            current_class: ClassType::None,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(top) = self.scopes.last_mut() {
            if let Some(_) = top.insert(name.lexeme.clone(), false) {
                error_token(name.clone(), "Already a variable with this name in this scope.");
                self.had_error = true;
            }
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name.lexeme.clone(), true);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_stmts(statements);
                self.end_scope();
            },
            Stmt::Class { name, methods } => {
                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;
                self.declare(name);
                self.define(name);
                self.begin_scope();
                self.scopes.last_mut().unwrap().insert("this".to_string(), true);
                for method in methods {
                    let mut declaration = FunctionType::Method;
                    if method.name.lexeme == "init" {
                        declaration = FunctionType::Initializer;
                    }
                    self.resolve_func(method, declaration);
                }
                self.end_scope();
                self.current_class = enclosing_class;
            },
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(init) = initializer {
                    self.resolve_expr(init);
                }
                self.define(name);
            },
            Stmt::Expression { expression } => {
                self.resolve_expr(expression);
            },
            Stmt::Function(function) => {
                self.declare(&function.name);
                self.define(&function.name);
                self.resolve_func(function, FunctionType::Function);
            },
            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if let Some(eb) = else_branch {
                    self.resolve_stmt(eb);
                }
            },
            Stmt::Print { expression } => {
                self.resolve_expr(expression);
            },
            Stmt::Return { keyword, value } => {
                if let Some(v) = value {
                    if self.current_function == FunctionType::Initializer {
                        error_token(keyword.clone(), "Can't return a value from an initializer.");
                    }
                    self.resolve_expr(v);
                }
            },
            Stmt::While { condition, body } => {
                self.resolve_expr(condition);
                self.resolve_stmt(body);
            },
        }
    }

    fn visit_expr(&mut self, expr: &ENode) {
        match &expr.expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(value);
                self.resolve_local(expr, name);
            },
            Expr::Binary { left, operator: _, right } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            },
            Expr::Call { callee, paren: _, arguments } => {
                self.resolve_expr(callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            },
            Expr::Get { object, name: _ } => {
                self.resolve_expr(object);
            },
            Expr::Grouping { expression } => {
                self.resolve_expr(expression);
            },
            Expr::Literal {..} => {},
            Expr::Logical { left, operator: _, right } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            },
            Expr::Set { object, name: _, value } => {
                self.resolve_expr(value);
                self.resolve_expr(object);
            },
            Expr::This { keyword } => {
                if self.current_class == ClassType::None {
                    error_token(keyword.clone(), "Can't use 'this' outside of a class.");
                    self.had_error = true;
                    return;
                }
                self.resolve_local(expr, keyword);
            }
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(right);
            },
            Expr::Variable { name } => {
                if let Some(top) = self.scopes.last() && let Some(e) = top.get(&name.lexeme) && !e {
                    error_token(name.clone(), "Can't read local variable in its own initializer");
                    self.had_error = true;
                }
                self.resolve_local(expr, name);
            },
        }
    }

    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        self.visit_stmt(stmt);
    }

    fn resolve_expr(&mut self, expr: &ENode) {
        self.visit_expr(expr);
    }

    fn resolve_local(&mut self, expr: &ENode, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, i);
                return;
            }
        }
    }

    fn resolve_func(&mut self, function: &Function, func_type: FunctionType) {
        let enclosing_func: FunctionType = self.current_function;
        self.current_function = func_type;
        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }
        for stmt in &function.body {
            self.resolve_stmt(stmt);
        }
        self.end_scope();
        self.current_function = enclosing_func;
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::resolver::Resolver;
    use crate::{parser::Parser, scanner::Scanner, token::Token};

    #[test]
    fn env_redefinition() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: env_redefinition =====");
        dbg!();
        let source_string = String::from(
"var a = \"global\";
{
  fun showA() {
    print a;
  }

  showA();
  var a = \"block\";
  showA();
}
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("global\nglobal\n", interpreter.result);
        Ok(())
    }
}
