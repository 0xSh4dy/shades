use crate::codegen::irgenerator::IrGenerator;
use crate::lexer::scanner::tokens::{Token, TokenList, TokenTypes};
use crate::lexer::variables::handle_var_decl;
use crate::utils::errors::fatal_error;
use inkwell::context::Context;
use inkwell::module::Module;

// Find if a string is a keyword
pub fn get_keyword(s: &str) -> TokenTypes {
    let res = s.chars().nth(0);
    if let Some(first_char) = res {
        match first_char {
            'p' => {
                if s == "print" {
                    return TokenTypes::T_PRINT;
                }
            }
            'v' => {
                if s == "var" {
                    return TokenTypes::T_VAR;
                }
            }
            's' => {
                if s == "string" {
                    return TokenTypes::T_STRING;
                }
            }
            'i' => {
                if s == "if"{
                    return TokenTypes::T_IF;
                }
            }
            'e' => {
                if s == "else"{
                    return TokenTypes::T_ELSE;
                }
            }
            
            _ => {}
        }
    }
    return TokenTypes::T_IDENTIF;
}


// pub fn match_and_generate<'a,'b>(
//     tokens: &mut TokenList,
//     context: &'a Context,
//     module: &'b Module<'a>,
// ) {
//     // Loop for handling everything within compound statements
//     loop {
//         let ir_gen = IrGenerator::new(&context, &module, "shades_main");
//         let cur_token = tokens.peek().unwrap();
//         let cur_token_type = cur_token.get_type();
//         if cur_token_type == TokenTypes::T_VAR {
//             tokens.next();
//             handle_var_decl(tokens);
//         } else if cur_token_type == TokenTypes::T_PRINT {
//             tokens.next();
//             ir_gen.handle_print_decl(tokens);
//         } else if cur_token_type == TokenTypes::T_IDENTIF {
//             ir_gen.handle_assignment(tokens);
//         }else if cur_token_type == TokenTypes::T_IF{
//             println!("if statement");
//             break;
//         }else if cur_token_type == TokenTypes::T_SEMICOLON{
//             // Skip the semicolon
//             tokens.next();
//         }
//         else if cur_token_type == TokenTypes::T_EOF{
//             break;
//         }
//     }
// }

pub fn match_lbrace(tokens:&mut TokenList){
    let cur_tok_opt = tokens.next();
    let res = match_token_internal(&cur_tok_opt,TokenTypes::T_LBRACE);
    if !res{
        fatal_error("Syntax error, compound statements must start with {", 1);
    }
}

pub fn match_token(cur_tok_opt:&Option<Token>,exp_type:TokenTypes){
    if !match_token_internal(cur_tok_opt,exp_type.clone()){
        fatal_error(&format!("Error! Couldn't find {:?}",exp_type),1)
    }
}

fn match_token_internal(cur_tok_opt:&Option<Token>,exp_type:TokenTypes)->bool {
    if let Some(cur_token) = cur_tok_opt{
        if cur_token.get_type() == exp_type{
            return true
        }
    }
    return false
}
