use crate::ast::astnode::{AstNode, AstOperation};

pub fn interpret_ast(root_node: Option<&Box<AstNode>>) -> i64 {
    if let Some(node) = root_node {
        if !(*node).is_leaf_node() {
            let left_val = interpret_ast((*node).get_left_child());
            let right_val = interpret_ast((*node).get_right_child());
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
            return (*node).get_intval();
        }
    }
    return 0;
}
