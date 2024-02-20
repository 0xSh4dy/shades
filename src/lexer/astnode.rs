use super::tokens::{Token, TokenTypes};
use crate::utils::errors::{fatal_error, throw_custom_error};
pub struct AstNode {
    op: AstOperation,
    left: Option<Box<AstNode>>,
    right: Option<Box<AstNode>>,
    intval: usize, // For integer value
}

// A type of AST Operation
pub enum AstOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Invalid,
}

impl AstNode {
    pub fn new() -> Box<AstNode> {
        return Box::new(AstNode {
            op: AstOperation::Invalid,
            left: None,
            right: None,
            intval: 0,
        });
    }

    pub fn create(
        op: AstOperation,
        left: Option<Box<AstNode>>,
        right: Option<Box<AstNode>>,
        intval: usize,
    ) -> Box<AstNode> {
        Box::new(AstNode {
            op,
            left,
            right,
            intval,
        })
    }
}

pub fn create_leaf_node(op: AstOperation, intval: usize) -> Box<AstNode> {
    Box::new(AstNode {
        op: op,
        left: None,
        right: None,
        intval: intval,
    })
}

pub fn create_unary_node(
    op: AstOperation,
    left: Option<Box<AstNode>>,
    intval: usize,
) -> Box<AstNode> {
    Box::new(AstNode {
        op: op,
        left: left,
        right: None,
        intval: intval,
    })
}

// Create a node for storing T_INTLIT (integer values)
pub fn create_primary_node(token: Token) -> Result<Box<AstNode>, Box<dyn std::error::Error>> {
    match token.get_type() {
        TokenTypes::T_INTLIT => {
            let ast_op = token.to_ast_operation();
            return Ok(create_leaf_node(ast_op, token.get_value()));
        }
        _ => {
            return Err(throw_custom_error("Syntax error"));
        }
    }
}
