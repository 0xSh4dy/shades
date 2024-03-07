use crate::lexer::tokens::{Token, TokenTypes};
use crate::utils::errors::fatal_error;

#[derive(PartialEq)]
pub struct AstNode {
    op: AstOperation,
    left: Option<Box<AstNode>>,
    right: Option<Box<AstNode>>,
    intval: usize, // For integer value
}

// A type of AST Operation
#[derive(Clone, Debug, PartialEq)]
pub enum AstOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Intlit,
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

    pub fn get_op(&self) -> AstOperation {
        self.op.clone()
    }

    pub fn get_intval(&self) -> i64 {
        self.intval as i64
    }

    
    pub fn get_left_child(&self) -> Option<&Box<AstNode>> {
        self.left.as_ref()
    }

    pub fn get_right_child(&self) -> Option<&Box<AstNode>> {
        self.right.as_ref()
    }

    pub fn is_leaf_node(&self) -> bool {
        return self.left == None && self.right == None;
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

#[allow(dead_code)]
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
pub fn create_primary_node(token: &Token) -> Box<AstNode> {
    match token.get_type() {
        TokenTypes::T_INTLIT => {
            let ast_op = token.to_ast_operation();
            return create_leaf_node(ast_op, token.get_value());
        }
        _ => {
            fatal_error("Syntax error in AST", 1);
            return AstNode::new();
        }
    }
}
