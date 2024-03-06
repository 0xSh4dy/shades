use crate::{ast::astnode::{AstNode, AstOperation}, llvm_wrappers::generators::mathgen::MathCodeGenerator};
use inkwell::{context::{Context, ContextRef}, module::Module};

pub struct IrGenerator<'a>{
    context:&'a Context,
    module:Module<'a>,
}

impl <'a> IrGenerator <'a>{
    pub fn new(context:&'a Context,module:Module<'a>)->IrGenerator<'a>{
        return IrGenerator { context:context,module: module };
    }

    pub unsafe fn generate_ir(&self,node_opt:Option<&Box<AstNode>>){
        if let Some(node) = node_opt{
            if !node.is_leaf_node(){
                let op = node.get_op();
                if op == AstOperation::Add{
                }
                else if op == AstOperation::Subtract{

                }
                else if op == AstOperation::Multiply{

                }
                else if op == AstOperation::Divide{

                }
            }
            else{

            }
        }
    }

    unsafe fn generate_ir_internal(&self){

    }
}