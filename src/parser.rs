use crate::ast::{BinaryOp, Expr, LiteralValue};
use crate::lexer::Token;
use logos::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Token::lexer(source);
        let current_token = lexer.next();
        Self {
            lexer,
            current_token,
        }
    }

    // This is the main entry point for parsing an expression.
    // It handles the lowest precedence: addition and subtraction.
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus => {
                    self.consume_token();
                    let right = self.parse_term()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.consume_token();
                    let right = self.parse_term()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Subtract,
                        right: Box::new(right),
                    };
                }
                _ => break, // No more + or - operators at this level
            }
        }
        Ok(left)
    }

    // Handles higher precedence: multiplication and division.
    fn parse_term(&mut self) -> Result<Expr, String> {
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
                _ => break, // No more * or / operators at this level
            }
        }
        Ok(left)
    }

    // Handles the highest precedence: numbers and parentheses.
    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.current_token.take() {
            Some(Token::Number(n)) => {
                self.consume_token();
                Ok(Expr::Literal(LiteralValue::Number(n)))
            }
            Some(Token::LParen) => {
                self.consume_token();
                let expr = self.parse_expression()?;
                if self.current_token != Some(Token::RParen) {
                    return Err("Expected closing parenthesis".to_string());
                }
                self.consume_token(); // Consume the ')'
                Ok(expr)
            }
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn consume_token(&mut self) {
        self.current_token = self.lexer.next();
    }
}