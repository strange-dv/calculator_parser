use crate::token::Token;
use crate::error::SyntaxError;

pub fn lex(code: String) -> Result<Vec<Token>, SyntaxError> {
    let mut iter = code.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut leftover: Option<char> = None;

    loop {
        let ch = match leftover {
            Some(ch) => ch,
            None => match iter.next() {
                None => break,
                Some(ch) => ch,
            },
        };
        leftover = None;
        match ch {
            ' ' => continue,
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '^' => tokens.push(Token::Carrot),
            ')' => tokens.push(Token::LeftParen),
            '(' => tokens.push(Token::RightParen),
            '-' => tokens.push(Token::Dash),
            ch if ch.is_ascii_digit() => {
                let number_stream: String = iter
                    .by_ref()
                    .take_while(|c| match c.is_ascii_digit() {
                        true => true,
                        false => {
                            leftover = Some(*c);
                            false
                        }
                    })
                    .collect();
                let number: i64 = format!("{}{}", ch, number_stream).parse().unwrap();
                tokens.push(Token::Number(number));
            }
            _ => {
                return Err(SyntaxError::new_lex_error(format!(
                    "Unrecognized character {}",
                    ch
                )))
            }
        }
    }

    tokens.push(Token::End);

    Ok(tokens)
}
