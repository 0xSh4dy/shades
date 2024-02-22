use crate::lexer::tokens::{TokenList, TokenTypes};

use super::{
    astnode::{create_primary_node, AstNode, AstOperation},
    oppred::get_precedence,
};

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

// Pratt Parser for handling operator precedence
// Node with higher operator precedence must stay lower in the tree
pub fn build_ast(tokens: &mut TokenList, prev_oppred: i32) -> Option<Box<AstNode>> {
    // Get the integer literal on the left and create a node
    if let Some(left_token) = tokens.next() {
        let mut left = create_primary_node(&left_token);
        if let Some(mut next_tok) = tokens.peek() {
            if next_tok.get_type() == TokenTypes::T_SEMICOLON {
                return Some(left);
            }
            while let Some(curr_pred) = get_precedence(next_tok.get_type()) {
                if curr_pred > prev_oppred {
                    tokens.next();
                    let right = build_ast(tokens, curr_pred);
                    let op = next_tok.to_ast_operation();
                    left = AstNode::create(op, Some(left), right, 0);
                    if let Some(x) = tokens.peek() {
                        if x.get_type() == TokenTypes::T_SEMICOLON {
                            return Some(left);
                        }
                        next_tok = x;
                    }
                } else {
                    break;
                }
            }
            return Some(left);
        }
    }
    return None;
}
