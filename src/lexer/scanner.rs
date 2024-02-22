use crate::{lexer::keywords::get_keyword, utils::errors::throw_custom_error};

use super::tokens::{Token, TokenList, TokenTypes};
// A tokenizer that extracts different tokens from an input string
pub struct Scanner {
    data: String,
    cur_idx: usize,
    tokens: Vec<Token>,
}

// Maximum allowed length of keywords or identifiers
const MAX_KILEN: i32 = 0x40;

#[allow(dead_code)]
impl Scanner {
    pub fn new(data: String) -> Scanner {
        Scanner {
            data: data,
            cur_idx: 0,
            tokens: Vec::new(),
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

    fn scan_char_sequence(&mut self, c: char) -> String {
        let mut ret_str = String::new();
        ret_str.push(c);
        self.advance();
        while self.has_next_token() {
            let cur_char = self.data.chars().nth(self.cur_idx).unwrap();
            if cur_char.is_ascii_alphanumeric() {
                ret_str.push(cur_char);
                self.advance();
            } else {
                self.stepback();
                break;
            }
        }
        return ret_str;
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
        let mut token_to_push = token.clone();

        if token_type == TokenTypes::T_INTLIT {
            let digit_string = self.scanint(token.get_value());
            let int_value: usize = digit_string.parse()?;
            token_to_push.set_value(int_value);
        } else if token_type == TokenTypes::T_INVALID {
            let val = std::char::from_u32(token.get_value() as u32).unwrap();
            if val.is_alphabetic() {
                let char_sequence = self.scan_char_sequence(val);
                let keyword_token = get_keyword(&char_sequence);
                if keyword_token == TokenTypes::T_PRINT {
                    token_to_push.set_type(keyword_token);
                }
            }
            else{
                let err_message = format!("Found invalid token {}", val);
                return Err(throw_custom_error(&err_message));
            }
        }
        self.tokens.push(token_to_push);
        Ok(())
    }

    pub fn has_next_token(&self) -> bool {
        self.cur_idx < self.data.len()
    }

    pub fn get_token_list(&self) -> Vec<Token> {
        return self.tokens.to_vec();
    }
}

pub fn start_scanner(filedata: String) -> Result<TokenList, Box<dyn std::error::Error>> {
    let mut scanner = Scanner::new(filedata);
    while scanner.has_next_token() {
        scanner.scan()?;
        scanner.advance()
    }
    scanner.tokens.push(Token::new_eof_token());

    Ok(TokenList::new(scanner.tokens))
}
