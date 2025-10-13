// src/evaluator.rs
use crate::ast::{BinaryOp, Expr, LiteralValue};

pub fn eval(expr: &Expr) -> Result<i64, String> {
    match expr {
        Expr::Literal(LiteralValue::Number(n)) => Ok(*n),
        Expr::Binary { left, op, right } => {
            let left_val = eval(left)?;
            let right_val = eval(right)?;
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