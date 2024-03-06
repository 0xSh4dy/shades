use super::{regops::RegOperations, x64::registers::X64Registers};
use crate::ast::astnode::{AstNode, AstOperation};

pub fn generate_asm(ast_root_opt: Option<&Box<AstNode>>, x64regs: &mut X64Registers) -> i32 {
    if let Some(node) = ast_root_opt {
        if !(*node).is_leaf_node() {
            let reg1 = generate_asm(node.get_left_child(), x64regs) as usize;
            let reg2 = generate_asm(node.get_right_child(), x64regs) as usize;
            let op = node.get_op();

            if op == AstOperation::Add {
                return x64regs.ro_add(reg1, reg2);
            } else if op == AstOperation::Subtract {
                return x64regs.ro_sub(reg1, reg2);
            } else if op == AstOperation::Multiply {
                return x64regs.ro_mul(reg1, reg2);
            } else if op == AstOperation::Divide {
                return x64regs.ro_div(reg1,reg2);
            }
        } else {
            return x64regs.ro_load((*node).get_intval());
        }
    }
    return 0;
}
