use std::ops::IndexMut;
use std::ptr::null;

use super::astnode::{create_primary_node, AstNode, AstOperation};
use crate::lexer::tokens::{Token, TokenList, TokenTypes};

#[allow(dead_code)]
pub fn in_order_traversal(root_node: Option<&Box<AstNode>>) {
    if let Some(node) = root_node {
        in_order_traversal((*node).get_left_child());
        let val = (*node).get_op();
        if val == AstOperation::Invalid {
            let value = (*node).get_intval();
            println!("{}", value);
        }
        in_order_traversal((*node).get_right_child());
    }
}

pub fn evaluate_binary_ast(root_node: Option<&Box<AstNode>>) -> u64 {
    if let Some(node) = root_node {
        if !(*node).is_leaf_node() {
            let left_val = evaluate_binary_ast((*node).get_left_child());
            let right_val = evaluate_binary_ast((*node).get_right_child());
            let op = (*node).get_op();
            if op == AstOperation::Add {
                return left_val + right_val;
            } else if op == AstOperation::Subtract {
                return left_val - right_val;
            } else if op == AstOperation::Multiply {
                return left_val * right_val;
            } else if op == AstOperation::Divide {
                return left_val / right_val;
            }
        } else {
            return (*node).get_actual_intval();
        }
    }
    return 0;
}

pub fn build_binary_ast_tree(mut tokens: TokenList) -> Option<Box<AstNode>> {
    let token_opt = tokens.next();
    if let Some(token) = token_opt {
        let left = create_primary_node(&token);
        let token_opt = tokens.next();
        if let Some(token) = token_opt {
            if token.get_type() == TokenTypes::T_EOF {
                return Some(left);
            }
            let optype = token.to_ast_operation();
            let right_opt = build_binary_ast_tree(tokens);
            return Some(AstNode::create(optype, Some(left), right_opt, 0));
        }
    }
    return None;
}
