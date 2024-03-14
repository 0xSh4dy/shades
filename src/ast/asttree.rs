use crate::{
    lexer::{
        scanner::{
            matcher::match_token,
            tokens::{TokenList, TokenTypes, TokenValue},
        },
        symbols::symtab::find_symbol,
    },
    utils::errors::fatal_error,
};

use super::{
    astnode::{
        create_leaf_node, create_primary_node, create_unary_node, AstNode, AstOperation, Value,
    },
    oppred::get_precedence,
};

#[allow(dead_code)]
pub fn in_order_traversal(root_node: Option<&Box<AstNode>>) {
    if let Some(node) = root_node {
        in_order_traversal((*node).get_left_child());
        let val = (*node).get_op();
        if val == AstOperation::Invalid {
            let value = (*node).get_val();
            println!("Invalid value {:?}", value);
        }
        in_order_traversal((*node).get_right_child());
    }
}

// Pratt Parser for handling operator precedence
// Node with higher operator precedence must stay lower in the tree
pub fn build_expression_tree(tokens: &mut TokenList, prev_oppred: i32) -> Option<Box<AstNode>> {
    // Get the integer literal on the left and create a node
    if let Some(left_token) = tokens.next() {
        let mut left = create_primary_node(&left_token);
        if let Some(mut next_tok) = tokens.peek() {
            let next_tok_type = next_tok.get_type();
            if next_tok_type == TokenTypes::T_SEMICOLON || next_tok_type == TokenTypes::T_LSMBRACE {
                return Some(left);
            }
            while let Some(curr_pred) = get_precedence(next_tok.get_type()) {
                if curr_pred > prev_oppred {
                    tokens.next();
                    let right = build_expression_tree(tokens, curr_pred);
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

// Build the tree for print statements
pub fn build_print_tree(tokens: &mut TokenList) -> Box<AstNode> {
    // Check if the first token is print or not
    match_token(&tokens.next(), TokenTypes::T_PRINT);
    // Evaluate the expression
    let root = build_expression_tree(tokens, 0);
    // Check for semicolon
    match_token(&tokens.next(), TokenTypes::T_SEMICOLON);
    return create_unary_node(AstOperation::Print, root, 0);
}

// Build the tree for assignment statements
pub fn build_assignment_tree(tokens: &mut TokenList) -> Box<AstNode> {
    // The first token must be an identifier
    let first_token = tokens.next();
    match_token(&first_token, TokenTypes::T_IDENTIF);
    // Check if the variable has been already declared
    if let TokenValue::String(var_name) = first_token.unwrap().get_value() {
        if let Some(idx) = find_symbol(&var_name) {
            match_token(&tokens.next(), TokenTypes::T_EQUAL);
            let right = create_leaf_node(AstOperation::Lvident, Value::SlotNumber(idx));
            let left = build_expression_tree(tokens, 0);
            let tree = AstNode::create(AstOperation::Assign, left, Some(right), 0);
            return tree;
        } else {
            fatal_error(&format!("Use of undeclared variable {}", var_name), 1);
        }
    }
    panic!("Error! Couldn't build the assignment AST");
}

pub fn build_if_tree(tokens: &mut TokenList) {
    match_token(&tokens.next(), TokenTypes::T_IF);
    match_token(&tokens.next(), TokenTypes::T_LSMBRACE);
    let expr_tree_opt = build_expression_tree(tokens, 0);
    if let Some(expr_tree) = expr_tree_opt {
        let op = expr_tree.get_op();
        if op < AstOperation::LessThan || op > AstOperation::NotEqual {
            fatal_error("Invalid comparison",1);
        }
        match_token(&tokens.next(),TokenTypes::T_RSMBRACE);
        
    }
}
