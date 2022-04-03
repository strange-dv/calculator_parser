use crate::error::SyntaxError;
use crate::expression::Expression;
use crate::operator::Operator;
use crate::token::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        Parser { iter }
    }

    fn assert_next(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();
        if let None = next {
            return Err(SyntaxError::new_parse_error(
                "Unexpected end of input".to_string(),
            ));
        }

        if *next.unwrap() != token {
            return Err(SyntaxError::new_parse_error(format!(
                "Expected {:?} actual {:?}",
                token,
                next.unwrap(),
            )));
        }

        Ok(())
    }

    pub(crate) fn parse(&mut self) -> Result<Expression, SyntaxError> {
        let ast = self.expression()?;
        self.assert_next(Token::End)?;
        Ok(ast)
    }

    fn primary(&mut self) -> Result<Expression, SyntaxError> {
        let next = self.iter.next().unwrap();

        match next {
            Token::Number(n) => Ok(Expression::Number(*n)),
            Token::RightParen => {
                let expr = self.expression()?;
                self.assert_next(Token::LeftParen)?;
                Ok(expr)
            }
            Token::Dash => {
                let expr = self.factor()?;
                Ok(Expression::Unary(Operator::Subtract, Box::new(expr)))
            }
            _ => Err(SyntaxError::new_parse_error(format!(
                "Unexpected token {:?}",
                next
            ))),
        }
    }

    fn factor(&mut self) -> Result<Expression, SyntaxError> {
        let expr = self.primary()?;
        let next = self.iter.peek().unwrap();
        if *next == &Token::Carrot {
            self.iter.next();
            let rhs = self.factor()?;
            return Ok(Expression::Binary(
                Operator::Power,
                Box::new(expr),
                Box::new(rhs),
            ));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, SyntaxError> {
        let mut expr: Expression = self.factor()?;

        loop {
            let next = self.iter.peek().unwrap();
            match next {
                Token::Star => {
                    self.iter.next();
                    let rhs = self.factor()?;
                    expr = Expression::Binary(Operator::Multiple, Box::new(expr), Box::new(rhs));
                }
                Token::Slash => {
                    self.iter.next();
                    let rhs = self.factor()?;
                    expr = Expression::Binary(Operator::Divide, Box::new(expr), Box::new(rhs));
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut expr: Expression = self.term()?;

        loop {
            let next = self.iter.peek().unwrap();
            match next {
                Token::Plus => {
                    self.iter.next();
                    let rhs = self.term()?;
                    expr = Expression::Binary(Operator::Add, Box::new(expr), Box::new(rhs));
                }
                Token::Dash => {
                    self.iter.next();
                    let rhs = self.term()?;
                    expr = Expression::Binary(Operator::Subtract, Box::new(expr), Box::new(rhs));
                }
                _ => break,
            };
        }

        Ok(expr)
    }
}
