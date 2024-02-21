use crate::ast::{
    astnode::AstNode,
    asttree::{self, evaluate_binary_ast},
};
use crate::lexer::tokens::TokenList;

pub fn generate_code(tokens: TokenList) {
    let root = asttree::build_binary_ast_tree(tokens);
    let val = asttree::evaluate_binary_ast(root.as_ref());
    println!("{}", val);
}
