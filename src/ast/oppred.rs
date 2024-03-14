use std::collections::HashMap;

use crate::{lexer::scanner::tokens::TokenTypes, utils::errors::fatal_error};

struct OperatorPrecedence{
    data:HashMap<TokenTypes,i32>
}


impl OperatorPrecedence{
    pub fn new()->OperatorPrecedence{
        let mut data_map:HashMap<TokenTypes,i32> = HashMap::new();
        data_map.insert(TokenTypes::T_PLUS,30);
        data_map.insert(TokenTypes::T_MINUS,30);
        data_map.insert(TokenTypes::T_STAR,50);
        data_map.insert(TokenTypes::T_SLASH,50);
        data_map.insert(TokenTypes::T_LT,5);
        data_map.insert(TokenTypes::T_LTEQ,5);
        data_map.insert(TokenTypes::T_GT,5);
        data_map.insert(TokenTypes::T_GTEQ,5);
        data_map.insert(TokenTypes::T_CEQ,10);
        data_map.insert(TokenTypes::T_NEQ,10);

        OperatorPrecedence { data: data_map }
    }
    fn get_precedence(&self,token_type:TokenTypes)->i32{
        let res = self.data.get(&token_type).cloned();
        match res{
            Some(val)=>return val,
            None => {
                fatal_error("Syntax error 1", 1);
                return -1;
            }
        }
    }
}

pub fn get_precedence(token_type:TokenTypes)->Option<i32>{
    let op_pred = OperatorPrecedence::new();
    Some(op_pred.get_precedence(token_type))
}