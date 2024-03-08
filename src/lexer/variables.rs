use crate::utils::errors::fatal_error;

use super::{symbols::symtab::add_symbol, tokens::{Token, TokenList, TokenTypes, TokenValue}};

pub fn handle_var_decl(tokens:&mut TokenList){
    // The next token must be an identifier;
    let next_token_opt = tokens.next();
    if let Some(next_token) = next_token_opt{
        if next_token.get_type() == TokenTypes::T_IDENTIF{
            if let TokenValue::String(val) = next_token.get_value(){
                add_symbol(val);
            }
            if let Some(next_token) = tokens.next(){
                if next_token.get_type() != TokenTypes::T_SEMICOLON{
                    fatal_error("Syntax error",1);
                }
            }
        }
        else{
            fatal_error("Syntax error", 1);
        }
    }
}