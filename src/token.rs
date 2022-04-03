#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Plus,
    Dash,
    Star,
    Slash,
    Carrot,
    LeftParen,
    RightParen,
    End,
    Number(i64),
}
