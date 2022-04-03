use std::error::Error;

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    level: String,
}

impl SyntaxError {
    pub fn new_lex_error(msg: String) -> Self {
        SyntaxError {
            message: msg,
            level: "Lex".to_string(),
        }
    }

    pub fn new_parse_error(msg: String) -> Self {
        SyntaxError {
            message: msg,
            level: "Parse".to_string(),
        }
    }
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

impl Error for SyntaxError {}
