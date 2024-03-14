use inkwell::{context::Context, module::Module};

use crate::{ast::asttree::{build_assignment_tree, build_if_tree, build_print_tree}, lexer::variables::handle_var_decl};

use super::{matcher::match_lbrace, tokens::{TokenList, TokenTypes}};

pub fn handle_compound_statement<'a,'b>(
    tokens:&mut TokenList,
    context:&'a  Context,
    module:&'b Module<'a>
){
    match_lbrace(tokens);
    loop{
        let cur_tok_opt = tokens.peek();
        if let Some(cur_tok) = cur_tok_opt{
            let token_type = cur_tok.get_type();
            match token_type{
                TokenTypes::T_PRINT => {
                    build_print_tree(tokens);
                },
                TokenTypes::T_IF => {
                    build_if_tree(tokens);
                },
                TokenTypes::T_VAR => {
                    handle_var_decl(tokens);
                },
                TokenTypes::T_IDENTIF => {
                    build_assignment_tree(tokens);
                },
                TokenTypes::T_RBRACE => {

                },
                _ => {}
            }
        }
        else{
            break;
        }
    }
}