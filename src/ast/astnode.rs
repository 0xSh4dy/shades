use crate::lexer::scanner::tokens::{Token, TokenTypes, TokenValue};
use crate::lexer::symbols::symtab::find_symbol;
use crate::utils::errors::fatal_error;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Intval(usize),
    SlotNumber(usize),
}
#[derive(PartialEq, Clone, Debug)]
pub struct AstNode {
    op: AstOperation,
    left: Option<Box<AstNode>>,
    mid: Option<Box<AstNode>>,
    right: Option<Box<AstNode>>,
    val: Value, // For integer value,
}

// A type of AST Operation
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum AstOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Intlit,
    Assign,
    Lvident,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    Equal,
    NotEqual,
    Print,
    Glue,
    If,
    While,
    Invalid,
}

impl AstNode {
    pub fn new() -> Box<AstNode> {
        return Box::new(AstNode {
            op: AstOperation::Invalid,
            left: None,
            mid: None,
            right: None,
            val: Value::Intval(0),
        });
    }

    pub fn get_op(&self) -> AstOperation {
        self.op.clone()
    }

    pub fn get_val(&self) -> Value {
        return self.val.clone();
    }

    pub fn get_left_child(&self) -> Option<&Box<AstNode>> {
        self.left.as_ref()
    }

    pub fn get_mid_child(&self) -> Option<&Box<AstNode>> {
        self.mid.as_ref()
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
        mid: Option<Box<AstNode>>,
        right: Option<Box<AstNode>>,
        intval: usize,
    ) -> Box<AstNode> {
        Box::new(AstNode {
            op,
            left,
            mid,
            right,
            val: Value::Intval(intval),
        })
    }
}

pub fn create_leaf_node(op: AstOperation, val: Value) -> Box<AstNode> {
    Box::new(AstNode {
        op: op,
        left: None,
        mid: None,
        right: None,
        val,
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
        mid: None,
        right: None,
        val: Value::Intval(intval),
    })
}

// Create a node for storing T_INTLIT (integer values)
pub fn create_primary_node(token: &Token) -> Box<AstNode> {
    match token.get_type() {
        TokenTypes::T_INTLIT => {
            let ast_op = token.to_ast_operation();
            if let TokenValue::Integer(v) = token.get_value() {
                return create_leaf_node(ast_op, Value::Intval(v));
            }
            return create_leaf_node(ast_op, Value::Intval(0));
        }
        TokenTypes::T_IDENTIF => {
            let ast_op = token.to_ast_operation();
            if let TokenValue::String(x) = token.get_value() {
                let sym_off = find_symbol(&x).expect("Symbol lookup failed");
                return create_leaf_node(ast_op, Value::SlotNumber(sym_off));
            }
            fatal_error("Error in create_primary_node", 1);
            return create_leaf_node(ast_op, Value::SlotNumber(0));
        }
        _ => {
            let err = format!("Syntax error in AST: Invalid token {:?}", token.get_type());
            fatal_error(&err, 1);
            return AstNode::new();
        }
    }
}
