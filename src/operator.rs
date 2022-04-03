use crate::token;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiple,
    Divide,
    Power,
}

impl TryFrom<token::Token> for Operator {
    type Error = &'static str;

    fn try_from(value: token::Token) -> Result<Self, Self::Error> {
        match value {
            token::Token::Plus => Ok(Operator::Add),
            token::Token::Dash => Ok(Operator::Subtract),
            token::Token::Star => Ok(Operator::Multiple),
            token::Token::Slash => Ok(Operator::Divide),
            token::Token::Carrot => Ok(Operator::Power),
            _ => Err("Only operators are supported"),
        }
    }
}
