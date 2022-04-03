use crate::operator::Operator;

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Number(i64),
}

impl Expression {
    pub(crate) fn eval(&mut self) -> i64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Unary(_negative, expr) => -1 * expr.eval(),
            Expression::Binary(Operator::Add, expr1, expr2) => expr1.eval() + expr2.eval(),
            Expression::Binary(Operator::Subtract, expr1, expr2) => expr1.eval() - expr2.eval(),
            Expression::Binary(Operator::Multiple, expr1, expr2) => expr1.eval() * expr2.eval(),
            Expression::Binary(Operator::Divide, expr1, expr2) => expr1.eval() / expr2.eval(),
            Expression::Binary(Operator::Power, expr1, expr2) => {
                expr1.eval().pow(expr2.eval() as u32)
            }
        }
    }
}
