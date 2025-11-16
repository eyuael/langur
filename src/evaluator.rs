use crate::ast::{BinaryOp, Expr, LiteralValue};
use std::collections::HashMap;

// The Environment (symbol table) stores variable names and their values
pub struct Environment {
    variables: HashMap<String, i64>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: i64) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<i64, String> {
        self.variables
            .get(name)
            .copied()
            .ok_or_else(|| format!("Undefined variable: {}", name))
    }
}

// Main evaluation function
pub fn eval(expr: &Expr, env: &mut Environment) -> Result<i64, String> {
    match expr {
        Expr::Literal(LiteralValue::Number(n)) => Ok(*n),
        
        // New: Handle variable lookup
        Expr::Variable(name) => env.get(name),
        
        // New: Handle assignment
        Expr::Assign { name, value } => {
            // First evaluate the right side
            let val = eval(value, env)?;
            // Then store it in the environment
            env.set(name.clone(), val);
            // Assignment expressions return the assigned value
            Ok(val)
        }
        
        Expr::Binary { left, op, right } => {
            let left_val = eval(left, env)?;
            let right_val = eval(right, env)?;
            match op {
                BinaryOp::Add => Ok(left_val + right_val),
                BinaryOp::Subtract => Ok(left_val - right_val),
                BinaryOp::Multiply => Ok(left_val * right_val),
                BinaryOp::Divide => {
                    if right_val == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
            }
        }
    }
}