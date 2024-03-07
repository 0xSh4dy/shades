use crate::lexer::common::handle_print_decl;
use crate::lexer::tokens::{TokenList, TokenTypes};
use crate::lexer::variables::handle_var_decl;
use crate::utils::errors::fatal_error;

// Find if a string is a keyword
pub fn get_keyword(s: &str) -> TokenTypes {
    let res = s.chars().nth(0);
    if let Some(first_char) = res {
        match first_char {
            'p' => {
                if s == "print" {
                    return TokenTypes::T_PRINT;
                }
            },
            'v' => {
                if s == "var" {
                    return TokenTypes::T_VAR;
                }
            },
            's' => {
                if s == "string"{
                    return TokenTypes::T_STRING;
                }
            },
            _ => {}
        }
    }
    return TokenTypes::T_IDENTIF;
}

pub fn match_items(tokens: &mut TokenList) {
    loop {
        let cur_token = tokens.peek().unwrap().get_type();
        if cur_token == TokenTypes::T_VAR {
            tokens.next();
            handle_var_decl(tokens);
        } else if cur_token == TokenTypes::T_PRINT {
            tokens.next();
            handle_print_decl();
        } 
        else{
            break;
        }
    }
}

pub fn match_keyword(t: TokenTypes) {
    if t == TokenTypes::T_PRINT {
    } else if t == TokenTypes::T_VAR {
    } else if t == TokenTypes::T_IDENTIF {
    } else {
        fatal_error("Syntax error right here", 1);
    }
}
