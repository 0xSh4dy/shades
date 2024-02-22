use crate::utils::errors::fatal_error;

use super::tokens::TokenTypes;

// Find if a string is a keyword
pub fn get_keyword(s:&str)->TokenTypes{
    let res = s.chars().nth(0);
    if let Some(first_char) = res{
        match first_char{
            'p'=>{
                if s == "print"{
                    return TokenTypes::T_PRINT;
                }
            },
            _=>{}
        }
    }
    return TokenTypes::T_INVALID;
}

pub fn match_keyword(t:TokenTypes){
    if t != TokenTypes::T_PRINT{
        fatal_error("Syntax error! Must start with print", 1);
    }
}