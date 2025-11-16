use crate::ast::{BinaryOp, Expr, LiteralValue};
use crate::lexer::Token;
use logos::{Lexer, Logos};

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Token::lexer(source);
        let current_token = lexer.next().and_then(|r| r.ok());
        Self {
            lexer,
            current_token,
        }
    }

    // Main entry point. Now handles assignments at the lowest precedence level.
    // This allows: `x = 5 + 3` to parse correctly
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        let expr = self.parse_assignment()?;
        Ok(expr)
    }

    // New: Handle assignment expressions (lowest precedence)
    // Assignments like: x = 10, y = x + 5
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or_expr()?;

        // Check if the next token is an equals sign
        if let Some(Token::Assign) = &self.current_token {
            // The left side must be a variable
            match expr {
                Expr::Variable(name) => {
                    self.consume_token(); // Consume the '='
                    let value = self.parse_assignment()?; // Right-associative: a = b = 5
                    Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => Err("Left side of assignment must be a variable".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    // Placeholder for future: could add logical operators (||, &&) here
    fn parse_or_expr(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    // Handles addition and subtraction (lower precedence than multiplication)
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus => {
                    self.consume_token();
                    let right = self.parse_multiplicative()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.consume_token();
                    let right = self.parse_multiplicative()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Subtract,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // Handles multiplication and division (higher precedence)
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Star => {
                    self.consume_token();
                    let right = self.parse_factor()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Multiply,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.consume_token();
                    let right = self.parse_factor()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Divide,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // Handles highest precedence: literals, variables, and parentheses
    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.current_token.take() {
            Some(Token::Number(n)) => {
                self.consume_token();
                Ok(Expr::Literal(LiteralValue::Number(n)))
            }
            Some(Token::Identifier(name)) => {
                self.consume_token();
                Ok(Expr::Variable(name))
            }
            Some(Token::LParen) => {
                self.consume_token();
                let expr = self.parse_expression()?;
                if self.current_token != Some(Token::RParen) {
                    return Err("Expected closing parenthesis".to_string());
                }
                self.consume_token();
                Ok(expr)
            }
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn consume_token(&mut self) {
        self.current_token = self.lexer.next().and_then(|r| r.ok());
    }
}