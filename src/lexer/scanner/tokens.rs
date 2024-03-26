use crate::ast::astnode::AstOperation;
use std::collections::VecDeque;

#[derive(Clone,Debug)]
pub enum TokenValue{
    Integer(usize),
    String(String),
}
// A token is a terminal symbol
#[derive(Clone,Debug)]
pub struct Token {
    token_type: TokenTypes,
    value: TokenValue,
}

// The token field can be any one of the following values
#[derive(Clone, PartialEq, Debug,Eq,Hash)]
#[allow(non_camel_case_types)]
pub enum TokenTypes {
    T_INTLIT,  // integer literal
    T_PLUS,    // plus symbol
    T_MINUS,   // minus symbol
    T_STAR,    // star symbol
    T_SLASH,   // slash symbol,
    T_EQUAL,   // equal symbol (=),
    T_PRINT,   // print keyword
    T_FUNC,    // func keyword
    T_STRUCT,  // struct keyword,
    T_STRING,  // string keyword
    T_VAR,     // var keyword
    T_SEMICOLON, // semicolon
    T_INVALID, // invalid token
    T_IDENTIF, // identifier token
    T_IF, // if
    T_ELSE, // else
    T_FOR, // for loop
    T_LOOP, // infinite loop
    T_WHILE, // while loop
    T_BREAK, // break statement
    T_CEQ, // ==
    T_NEQ, // !=
    T_GT,  // >
    T_LT,  // <
    T_GTEQ, // >=
    T_LTEQ, // <=
    T_NOT, // !
    T_LSMBRACE, // (
    T_RSMBRACE, // )
    T_LBRACE, // {
    T_RBRACE, // }
    T_EOF,     // End of file
}

#[derive(Debug)]
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
    pub fn peek(&mut self)->Option<Token>{
        self.tokens.front().cloned()
    }
}

#[allow(unused_assignments)]
impl Token {
    pub fn new_eof_token() -> Token {
        Token {
            token_type: TokenTypes::T_EOF,
            value: TokenValue::Integer(0),
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
            ';' => token_type = TokenTypes::T_SEMICOLON,
            '=' => token_type = TokenTypes::T_EQUAL,
            '<' => token_type = TokenTypes::T_LT,
            '>' => token_type = TokenTypes::T_GT,
            '!' => token_type = TokenTypes::T_NOT,
            '(' => token_type = TokenTypes::T_LSMBRACE,
            ')' => token_type = TokenTypes::T_RSMBRACE,
            '{' => token_type = TokenTypes::T_LBRACE,
            '}' => token_type = TokenTypes::T_RBRACE,
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
            value: TokenValue::Integer(token_value),
        }
    }

    pub fn new_multichar_token(chars:&str)->Token{
        let mut token_type = TokenTypes::T_INVALID;
        let token_value:usize = 0;

        if chars == "=="{
            token_type = TokenTypes::T_CEQ;
        }
        else if chars == "!="{
            token_type = TokenTypes::T_NEQ
        }
        else if chars == ">="{
            token_type = TokenTypes::T_GTEQ;
        }
        else if chars == "<="{
            token_type = TokenTypes::T_LTEQ;
        }
        return Token{
            token_type:token_type,
            value:TokenValue::Integer(token_value)
        };
    }
    pub fn get_type(&self) -> TokenTypes {
        self.token_type.clone()
    }

    pub fn get_value(&self) -> TokenValue {
        self.value.clone()
    }

    pub fn set_type(&mut self,t:TokenTypes){
        self.token_type = t;
    }
    pub fn set_value(&mut self,value:TokenValue){
        self.value = value;
    }
    pub fn to_ast_operation(&self) -> AstOperation {
        let token_type = self.token_type.clone();
        match token_type {
            TokenTypes::T_PLUS => AstOperation::Add,
            TokenTypes::T_MINUS => AstOperation::Subtract,
            TokenTypes::T_STAR => AstOperation::Multiply,
            TokenTypes::T_SLASH => AstOperation::Divide,
            TokenTypes::T_INTLIT => AstOperation::Intlit,
            TokenTypes::T_CEQ => AstOperation::Equal ,
            TokenTypes::T_GT => AstOperation::GreaterThan,
            TokenTypes::T_LT => AstOperation::LessThan,
            TokenTypes::T_LTEQ => AstOperation::LessThanEq,
            TokenTypes::T_NEQ => AstOperation::NotEqual,
            TokenTypes::T_GTEQ => AstOperation::GreaterThanEq,
            TokenTypes::T_IDENTIF => AstOperation::Lvident,
            _ => AstOperation::Invalid,
        }
    }
}
