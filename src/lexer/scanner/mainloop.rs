use inkwell::{context::Context, module::Module};

use crate::{
    ast::{
        astnode::{AstNode, AstOperation},
        asttree::{build_assignment_tree, build_if_tree, build_print_tree, build_while_tree},
    },
    lexer::variables::handle_var_decl,
};

use super::{
    matcher::{match_lbrace, match_token},
    tokens::{TokenList, TokenTypes},
};

pub fn handle_compound_statement<'a, 'b>(
    tokens: &mut TokenList,
) -> Option<Box<AstNode>> {
    let mut tree: Option<Box<AstNode>> = None;
    let mut left: Option<Box<AstNode>> = None;
    match_lbrace(tokens);
    loop {
        let cur_tok_opt = tokens.peek();
        if let Some(cur_tok) = cur_tok_opt {
            let token_type = cur_tok.get_type();
            match token_type {
                TokenTypes::T_PRINT => {
                    println!("Generating tree for print");
                    tree = Some(build_print_tree(tokens));
                }
                TokenTypes::T_IF => {
                    println!("Generating tree for if");
                    tree = Some(build_if_tree(tokens));
                }
                TokenTypes::T_WHILE => {
                    println!("Generating tree for while");
                    tree = Some(build_while_tree(tokens));
                }
                TokenTypes::T_VAR => {
                    println!("Handling variable declaration");
                    handle_var_decl(tokens);
                }
                TokenTypes::T_IDENTIF => {
                    println!("Generating assignment tree");
                    tree = Some(build_assignment_tree(tokens));
                }
                TokenTypes::T_RBRACE => {
                    tokens.next();
                    return left;
                }
                _ => {
                    println!("{:?}",token_type);
                    panic!("detected syntax error when building AST")
                }
            }
        } else {
            panic!("encountered unexpected error when building tree for compound statements")
        }
        if let Some(tree_node) = tree.clone() {
            if let Some(left_node) = left {
                left = Some(AstNode::create(AstOperation::Glue, Some(left_node), None, Some(tree_node), 0));
            } else {
                left = Some(tree_node);
            }
        }
    }

}
