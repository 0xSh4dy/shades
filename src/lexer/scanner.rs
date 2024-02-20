use super::tokens::{Token, TokenTypes};
use crate::utils::errors::throw_custom_error;
// A tokenizer that extracts different tokens from an input string
pub struct Scanner {
    data: String,
    cur_idx: usize,
}

impl Scanner {
    pub fn new(data: String) -> Scanner {
        Scanner {
            data: data,
            cur_idx: 0,
        }
    }

    fn get_next_token(&mut self) -> Token {
        loop {
            let res = self.data.chars().nth(self.cur_idx).unwrap();
            if !res.is_whitespace() {
                return Token::new(res);
            }
            self.advance()
        }
    }

    fn scanint(&mut self, curr_val: usize) -> String {
        let mut digit_string = String::new();
        let res = std::char::from_u32(curr_val as u32);
        if let Some(val) = res {
            digit_string.push(val);
        }
        self.advance();

        while self.has_next_token() {
            let cur_char = self.data.chars().nth(self.cur_idx).unwrap();
            if cur_char.is_digit(10) {
                digit_string.push(cur_char);
                self.advance();
            } else {
                self.stepback();
                break;
            }
        }

        return digit_string;
    }

    fn stepback(&mut self) {
        self.cur_idx -= 1;
    }

    fn advance(&mut self) {
        self.cur_idx += 1;
    }

    fn scan(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.get_next_token();
        let token_type = token.get_type();
        if token_type == TokenTypes::T_INTLIT {
            let digit_string = self.scanint(token.get_value());
            let val = digit_string.parse::<u64>()?;
            println!("{} , {}", token_type.to_string(), val);
        } else if token_type == TokenTypes::T_INVALID {
            let val = std::char::from_u32(token.get_value() as u32).unwrap();
            let err_message = format!("Found invalid token {}", val);
            return Err(throw_custom_error(&err_message));
        } else {
            println!("{}", token_type.to_string());
        }
        Ok(())
    }

    pub fn has_next_token(&self) -> bool {
        self.cur_idx < self.data.len()
    }
}

pub fn start_scanner(filedata: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::new(filedata);
    while scanner.has_next_token() {
        scanner.scan()?;
        scanner.advance()
    }
    Ok(())
}
