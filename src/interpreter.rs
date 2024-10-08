use crate::ast::{Expr, Statement};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter {
    env: HashMap<String, i32>, // Environment to store variables and their values
    const_env: HashMap<String, i32>, // Separate environment for constants (val)
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
            const_env: HashMap::new(),
        }
    }

    pub fn eval_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.eval_statement(statement);
        }
    }

    fn eval_statement(&mut self, statement: Statement) {
        match statement {
            Statement::VarDecl(var_name, expr) => {
                let value = self.eval_expr(expr);
                if let EvalResult::Number(n) = value {
                    self.env.insert(var_name, n);
                } else {
                    panic!("Expected a number for variable declaration");
                }
            }
            Statement::ValDecl(var_name, expr) => {
                let value = self.eval_expr(expr);
                if let EvalResult::Number(n) = value {
                    if self.const_env.contains_key(&var_name) {
                        panic!("Constant '{}' already declared", var_name);
                    }
                    self.const_env.insert(var_name, n);
                } else {
                    panic!("Expected a number for constant declaration");
                }
            }
            Statement::Assign(var_name, expr) => {
                if self.const_env.contains_key(&var_name) {
                    panic!("Cannot reassign to constant '{}'", var_name);
                }
                if self.env.contains_key(&var_name) {
                    let value = self.eval_expr(expr);
                    if let EvalResult::Number(n) = value {
                        self.env.insert(var_name, n);
                    } else {
                        panic!("Expected a number for assignment");
                    }
                } else {
                    panic!("Undefined variable: {}", var_name);
                }
            }
            Statement::Print(expr) => {
                let result = self.eval_expr(expr);
                match result {
                    EvalResult::Number(value) => println!("{}", value), // Print numbers
                    EvalResult::StringLiteral(value) => println!("{}", value), // Print strings
                }
            }
        }
    }

    fn eval_expr(&self, expr: Expr) -> EvalResult {
        match expr {
            Expr::Number(n) => EvalResult::Number(n),
            Expr::StringLiteral(s) => EvalResult::StringLiteral(s),
            Expr::Variable(var_name) => {
                if let Some(value) = self.env.get(&var_name) {
                    EvalResult::Number(*value)
                } else if let Some(value) = self.const_env.get(&var_name) {
                    EvalResult::Number(*value)
                } else {
                    panic!("Undefined variable: {}", var_name);
                }
            }
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.eval_expr(*left).unwrap_number();
                let right_val = self.eval_expr(*right).unwrap_number();
                match op {
                    crate::ast::BinOp::Add => EvalResult::Number(left_val + right_val),
                    crate::ast::BinOp::Sub => EvalResult::Number(left_val - right_val),
                    crate::ast::BinOp::Mul => EvalResult::Number(left_val * right_val),
                    crate::ast::BinOp::Div => EvalResult::Number(left_val / right_val),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum EvalResult {
    Number(i32),
    StringLiteral(String),
}

impl EvalResult {
    pub fn unwrap_number(self) -> i32 {
        if let EvalResult::Number(n) = self {
            n
        } else {
            panic!("Expected a number, found {:?}", self);
        }
    }
}
