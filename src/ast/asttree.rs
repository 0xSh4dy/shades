use crate::{
    lexer::{
        scanner::{
            mainloop::handle_compound_statement, matcher::match_token, tokens::{TokenList, TokenTypes, TokenValue}
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
            if next_tok_type == TokenTypes::T_SEMICOLON  {
                // tokens.next();
                return Some(left);
            }else if next_tok_type == TokenTypes::T_RSMBRACE{
                println!("{:?}",left);
                return Some(left);
            }
            while let Some(curr_pred) = get_precedence(next_tok.get_type()) {
                if curr_pred > prev_oppred {
                    tokens.next();
                    let right = build_expression_tree(tokens, curr_pred);
                    let op = next_tok.to_ast_operation();
                    left = AstNode::create(op, Some(left),None, right, 0);
                    if let Some(x) = tokens.peek() {
                        if x.get_type() == TokenTypes::T_SEMICOLON || x.get_type()==TokenTypes::T_RSMBRACE {
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
    match_token(&tokens.next(), TokenTypes::T_PRINT);
    match_token(&tokens.next(),TokenTypes::T_LSMBRACE);
    let root = build_expression_tree(tokens, 0);
    match_token(&tokens.next(),TokenTypes::T_RSMBRACE);
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
            let tree = AstNode::create(AstOperation::Assign, left,None, Some(right), 0);
            match_token(&tokens.next(),TokenTypes::T_SEMICOLON);
            return tree;
        } else {
            fatal_error(&format!("Use of undeclared variable {}", var_name), 1);
        }
    }
    panic!("Error! Couldn't build the assignment AST");
}

pub fn build_if_tree(tokens: &mut TokenList) ->Box<AstNode>{
    match_token(&tokens.next(), TokenTypes::T_IF);
    match_token(&tokens.next(), TokenTypes::T_LSMBRACE);
    let expr_tree_opt = build_expression_tree(tokens, 0);
    if let Some(expr_tree) = expr_tree_opt {
        let op = expr_tree.get_op();
        if op < AstOperation::LessThan || op > AstOperation::NotEqual {
            fatal_error("Invalid comparison",1);
        }
        match_token(&tokens.next(),TokenTypes::T_RSMBRACE);
        let if_tree = handle_compound_statement(tokens);
        let next_token_opt = tokens.peek();
        if let Some(next_token) = next_token_opt{
            if next_token.get_type() == TokenTypes::T_ELSE{
                tokens.next();
                let else_tree = handle_compound_statement(tokens);
                return AstNode::create(AstOperation::If, Some(expr_tree), if_tree, else_tree,0)
            }
            return AstNode::create(AstOperation::If,Some(expr_tree),if_tree,None,0);
        }else{
            panic!("build_if_tree: failed to fetch next token");
        }
    }
    else{
        panic!("build_if_tree: failed to generate expression tree");
    }
}

pub fn build_while_tree(tokens:&mut TokenList)->Box<AstNode>{
    match_token(&tokens.next(),TokenTypes::T_WHILE);
    match_token(&tokens.next(),TokenTypes::T_LSMBRACE);
    let expr_tree_opt = build_expression_tree(tokens, 0);
    if let Some(expr_tree) = expr_tree_opt.clone(){
        let op = expr_tree.get_op();
        if op < AstOperation::LessThan || op > AstOperation::NotEqual{
            panic!("Invalid comparison in while loop");
        }
        match_token(&tokens.next(),TokenTypes::T_RSMBRACE);
        let inner_tree = handle_compound_statement(tokens);
        return AstNode::create(AstOperation::While, expr_tree_opt, None, inner_tree, 0);   
    }
    else{
        panic!("build_while_tree: failed to generate expression tree");
    }
}
