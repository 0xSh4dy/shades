use crate::ast::astnode::AstOperation;
use std::collections::VecDeque;
// A token is a terminal symbol
#[derive(Clone)]
pub struct Token {
    token_type: TokenTypes,
    value: usize,
}

// The token field can be any one of the following values
#[derive(Clone, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum TokenTypes {
    T_INTLIT,  // integer literal
    T_PLUS,    // plus symbol
    T_MINUS,   // minus symbol
    T_STAR,    // star symbol
    T_SLASH,   // slash symbol
    T_INVALID, // invalid token
    T_EOF,     // End of file
}

pub struct TokenList {
    tokens: VecDeque<Token>,
}

impl TokenList {
    pub fn new(tokens: Vec<Token>) -> TokenList {
        TokenList {
            tokens: VecDeque::from(tokens),
        }
    }
    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}
impl TokenTypes {
    pub fn to_string(&self) -> String {
        match *self {
            TokenTypes::T_INTLIT => "T_INTLIT".to_string(),
            TokenTypes::T_PLUS => "T_PLUS".to_string(),
            TokenTypes::T_MINUS => "T_MINUS".to_string(),
            TokenTypes::T_STAR => "T_STAR".to_string(),
            TokenTypes::T_SLASH => "T_SLASH".to_string(),
            TokenTypes::T_INVALID => "INVALID".to_string(),
            TokenTypes::T_EOF => "EOF".to_string(),
        }
    }
}

#[allow(unused_assignments)]
impl Token {
    pub fn new_eof_token() -> Token {
        Token {
            token_type: TokenTypes::T_EOF,
            value: 0,
        }
    }

    pub fn new(token: char) -> Token {
        let mut token_type = TokenTypes::T_INVALID;
        let mut token_value: usize = 0;

        match token {
            '-' => token_type = TokenTypes::T_MINUS,
            '+' => token_type = TokenTypes::T_PLUS,
            '*' => token_type = TokenTypes::T_STAR,
            '/' => token_type = TokenTypes::T_SLASH,
            _ => {
                token_value = token as usize;
                if token.is_digit(10) {
                    token_type = TokenTypes::T_INTLIT;
                } else {
                    token_type = TokenTypes::T_INVALID;
                }
            }
        }

        Token {
            token_type,
            value: token_value,
        }
    }

    pub fn get_type(&self) -> TokenTypes {
        self.token_type.clone()
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn to_ast_operation(&self) -> AstOperation {
        let token_type = self.token_type.clone();
        match token_type {
            TokenTypes::T_PLUS => AstOperation::Add,
            TokenTypes::T_MINUS => AstOperation::Subtract,
            TokenTypes::T_STAR => AstOperation::Multiply,
            TokenTypes::T_SLASH => AstOperation::Divide,
            _ => AstOperation::Invalid,
        }
    }
}
