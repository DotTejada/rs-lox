use std::collections::HashMap;

use crate::{error::{RuntimeError, error_token}, expr::ENode, expr::Expr, interpreter::Interpreter, stmt::{Function, Stmt}, token::Token};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
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
            top.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name.lexeme.clone(), true);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                for stmt in statements {
                    self.resolve_stmt(stmt)?;
                }
                self.end_scope();
            },
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(init) = initializer {
                    self.resolve_expr(init)?;
                }
                self.define(name);
            },
            Stmt::Expression { expression } => {
                self.resolve_expr(expression)?;
            },
            Stmt::Function(function) => {
                self.declare(&function.name);
                self.define(&function.name);
                self.resolve_func(function)?;
            },
            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(eb) = else_branch {
                    self.resolve_stmt(eb)?;
                }
            },
            Stmt::Print { expression } => {
                self.resolve_expr(expression)?;
            },
            Stmt::Return { keyword: _, value } => {
                if let Some(v) = value {
                    self.resolve_expr(v)?;
                }
            },
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            },
        }
        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(&value.expr)?;
                self.resolve_local(expr, name)?;
            },
            Expr::Binary { left, operator: _, right } => {
                self.resolve_expr(&left.expr)?;
                self.resolve_expr(&right.expr)?;
            },
            Expr::Call { callee, paren, arguments } => {
                self.resolve_expr(&callee.expr)?;
                for arg in arguments {
                    self.resolve_expr(&arg.expr)?;
                }
            },
            Expr::Grouping { expression } => {
                self.resolve_expr(&expression.expr)?;
            },
            Expr::Literal {..} => {},
            Expr::Logical { left, operator: _, right } => {
                self.resolve_expr(&left.expr)?;
                self.resolve_expr(&right.expr)?;
            },
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(&right.expr)?;
            },
            Expr::Variable { name } => {
                if let Some(top) = self.scopes.last() && let Some(e) = top.get(&name.lexeme) && !e {
                    error_token(name.clone(), "Can't read local variable in its own initializer");
                }
                self.resolve_local(expr, name)?;
            },
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        self.visit_stmt(stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        self.visit_expr(expr)
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn resolve_func(&mut self, function: &Function) -> Result<(), RuntimeError> {
        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }
        for stmt in &function.body {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();
        Ok(())
    }

}
