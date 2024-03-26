use crate::{
    ast::astnode::{AstNode, AstOperation, Value},
    lexer::symbols::symtab::get_corresp_symtab,
    llvm_wrappers::generators::{funcgen::FuncGenerator, mathgen::{self, ast_op_to_mathop, MathCodeGenerator, MathOps, FN_OPS}},
    utils::errors::fatal_error,
};
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::{Linkage, Module}, types::IntType, values::{BasicValue, BasicValueEnum, FunctionValue, IntValue}, IntPredicate
};

pub struct IrGenerator<'a, 'b> {
    context: &'a Context,
    module: &'b Module<'a>,
    parent_func: FunctionValue<'a>,
}

impl<'a, 'b> IrGenerator<'a, 'b> {
    pub fn new(
        context: &'a Context,
        module: &'b Module<'a>,
        parent_fn: &str,
    ) -> IrGenerator<'a, 'b> {
        let parent_func = module.get_function(parent_fn).unwrap();
        return IrGenerator {
            context: context,
            module: module,
            parent_func: parent_func,
        };
    }

    // pub fn generate_ir(&self,node_opt:Option<&Box<AstNode>>){
    //     let mut builder = self.context.create_builder();
    //     let entry_block = self.context.append_basic_block(self.parent_func,"entry");
    //     builder.position_at_end(entry_block);

        // let math_gen = mathgen::MathCodeGenerator::new(self.context,self.module);
        // math_gen.generate_add_int64();
        // let add_fn = self.module.get_function("shades_add").expect("failed to get_function");

        // let x = self.context.i32_type().const_int(5, false);
        // let y = self.context.i32_type().const_int(10, false);

        // let cond = self.context.bool_type().const_int(1,false);

        // let then_block = self.context.append_basic_block(self.parent_func,"then");
        // let else_block = self.context.append_basic_block(self.parent_func,"else");
        // let end_block = self.context.append_basic_block(self.parent_func,"end");

        // builder.build_conditional_branch(cond, then_block, else_block).expect("failed to build conditional branch");

        // builder.position_at_end(then_block);
        // // let then_val = builder.build_int_add(x, y, "sum");
        // builder.build_call(add_fn,&[x.into(),y.into()],"");
        // builder.build_unconditional_branch(end_block);

        // builder.position_at_end(else_block);
        // // let else_val = builder.build_int_sub(x, y, "diff");
        // builder.build_call(add_fn,&[x.into(),x.into()],"");
        // builder.build_unconditional_branch(end_block);
        
        // builder.position_at_end(end_block);
        // builder.build_return(None).expect("build_return failed");
    // }   

    pub fn generate_ir(&self,node_opt:Option<&Box<AstNode>>){
        let mut builder = self.context.create_builder();
        let entry_block = self.context.append_basic_block(self.parent_func,"entry");
        builder.position_at_end(entry_block);
        
        let math_gen = mathgen::MathCodeGenerator::new(self.context,self.module);
        math_gen.generate_add_int64();

        let first_bb = self.parent_func.get_first_basic_block().expect("failed to get the first basic block");
        self.generate_code(first_bb,node_opt);
        let last_bb = self.parent_func.get_last_basic_block().expect("failed to retrieve last basic block");
        builder.position_at_end(last_bb);
        builder.build_return(None).expect("build_return failed");
    }


    // fn generate_code(&self,builder:&mut Builder,node_opt:Option<&Box<AstNode>>)->(IntValue,bool){
    //     if let Some(node) = node_opt{
    //         if !node.is_leaf_node(){
    //             let op = node.get_op();

    //             if op == AstOperation::If{
    //                 let add_fn = self.module.get_function("shades_add").expect("failed to get_function");

    //                 let x = self.context.i32_type().const_int(5, false);
    //                 let y = self.context.i32_type().const_int(10, false);
            
    //                 let cond = self.context.bool_type().const_int(1,false);
            
    //                 let then_block = self.context.append_basic_block(self.parent_func,"then");
    //                 let else_block = self.context.append_basic_block(self.parent_func,"else");
    //                 let end_block = self.context.append_basic_block(self.parent_func,"end");
            
    //                 builder.build_conditional_branch(cond, then_block, else_block).expect("failed to build conditional branch");
            
    //                 builder.position_at_end(then_block);
    //                 // let then_val = builder.build_int_add(x, y, "sum");
    //                 builder.build_call(add_fn,&[x.into(),y.into()],"").expect("build_call failed");
    //                 builder.build_unconditional_branch(end_block).expect("uncond failed");
            
    //                 builder.position_at_end(else_block);
    //                 // let else_val = builder.build_int_sub(x, y, "diff");
    //                 builder.build_call(add_fn,&[x.into(),x.into()],"").expect("build_call failed");
    //                 builder.build_unconditional_branch(end_block).expect("uncond failed");
    //                 builder.position_at_end(end_block);
    //             }
    //             return (self.context.i64_type().const_int(0, false), false);
    //         }
    //         else{
    //             return (self.context.i64_type().const_int(0, false), false);
    //         }
    //     }
    //     panic!("node not found");
    // }

    fn generate_code(&self,block:BasicBlock,node_opt: Option<&Box<AstNode>>) -> (IntValue, bool) {
        if let Some(node) = node_opt {
            if !node.is_leaf_node() {
                let math_gen = MathCodeGenerator::new(&self.context, &self.module);
                let op = node.get_op();
                let mut builder = self.context.create_builder();
                let retval: IntValue;

                if op == AstOperation::If{
                    retval = self.context.i64_type().const_int(0,false);
                    // Get the left conditional tree
                    let cond_tree = node.get_left_child();
                    let then_tree = node.get_mid_child();
                    let else_tree = node.get_right_child();

                    let cond = self.generate_code(block,cond_tree).0;
                    
                    let then_block = self.context.append_basic_block(self.parent_func,"then");
                    let else_block = self.context.append_basic_block(self.parent_func,"else");
                    let ifcont_block = self.context.append_basic_block(self.parent_func,"ifcont");

                    // Build the conditional brancg
                    builder.position_at_end(block);
                    builder.build_conditional_branch(cond,then_block,else_block).expect("failed to build conditional branch");
                    
                    // Generate code for then block
                    builder.position_at_end(then_block);
                    self.generate_code(then_block,then_tree);
                    builder.build_unconditional_branch(ifcont_block).expect("failed to build unconditional branch");
              
                    // Generate code for else block
                    builder.position_at_end(else_block);
                    self.generate_code(else_block,else_tree);
                    builder.build_unconditional_branch(ifcont_block).expect("failed to build unconditional branch");

                    return (retval,false);
                }
                builder.position_at_end(block);

                let (left_val, slot_ret_left) = self.generate_code(block,node.get_left_child());
                let (right_val, slot_ret_right) = self.generate_code(block,node.get_right_child());


                let left_val = if slot_ret_left {
                    let symtab =
                        get_corresp_symtab(left_val.get_sign_extended_constant().unwrap() as usize);
                    let glob_value = self.module.get_global(&symtab.get_name()).expect(&format!(
                        "Variable {} might not have been initialized",
                        symtab.get_name()
                    ));
                    let ptr_value = glob_value.as_pointer_value();
                    let ptr_type = self.context.i64_type();
                    let loaded = builder
                        .build_load(ptr_type, ptr_value, &symtab.get_name())
                        .unwrap();
                    loaded
                } else {
                    left_val.as_basic_value_enum()
                };

                if op == AstOperation::Add {
                    println!("Generating code for Add");
                    math_gen.generate_add_int64();
                    let add_fn_name = FN_OPS.get(&MathOps::Add).unwrap();
                    let add_fn = self.module.get_function(add_fn_name).unwrap();

                    retval = builder
                        .build_call(add_fn, &[left_val.into(), right_val.into()], "add_retval")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap()
                        .into_int_value();
                } else if op == AstOperation::Subtract {
                    math_gen.generate_sub_int64();
                    let sub_fn_name = FN_OPS.get(&MathOps::Sub).unwrap();
                    let sub_fn = self.module.get_function(sub_fn_name).unwrap();
                    retval = builder
                        .build_call(sub_fn, &[left_val.into(), right_val.into()], "sub_retval")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap()
                        .into_int_value();
                } else if op == AstOperation::Multiply {
                    println!("Generating code for mul");
                    math_gen.generate_mul_int64();
                    let mul_fn_name = FN_OPS.get(&MathOps::Mul).unwrap();
                    let mul_fn = self.module.get_function(mul_fn_name).unwrap();
                    retval = builder
                        .build_call(mul_fn, &[left_val.into(), right_val.into()], "mul_retval")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap()
                        .into_int_value();
                } else if op == AstOperation::Divide {
                    math_gen.generate_div_int64();

                    let div_fn_name = FN_OPS.get(&MathOps::Div).unwrap();
                    let div_fn = self.module.get_function(div_fn_name).unwrap();
                    retval = builder
                        .build_call(div_fn, &[left_val.into(), right_val.into()], "div_retval")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap()
                        .into_int_value();
                } else if op == AstOperation::LessThan
                    || op == AstOperation::LessThanEq
                    || op == AstOperation::Equal
                    || op == AstOperation::NotEqual
                    || op == AstOperation::GreaterThan
                    || op == AstOperation::GreaterThanEq
                {
                    math_gen.generate_cmp_func(&op);
                    retval =
                        build_call_for_cmp(&self.module, &builder, right_val.into(), left_val, &op);
                } else if op == AstOperation::Assign {
                    retval = self.context.i64_type().const_int(0, false);
                    let slot_num = right_val.get_sign_extended_constant().unwrap();
                    let name = get_corresp_symtab(slot_num as usize).get_name();

                    let glob_var_opt = self.module.get_global(&name);
                    if let Some(glob_var) = glob_var_opt{
                        builder.build_store(glob_var.as_pointer_value(),left_val).expect("build_store failed");
                    }
                    else{
                        let glob_var = self.module.add_global(self.context.i64_type(),None,name.as_str());
                        glob_var.set_linkage(Linkage::Internal);
                        glob_var.set_initializer(&self.context.i64_type().const_int(0,false));
                        builder.build_store(glob_var.as_pointer_value(),left_val).expect("build_store failed");
                    }
                    // let glob_var =
                    //     self.module
                    //         .add_global(self.context.i64_type(), None, name.as_str());
                    // glob_var.set_linkage(Linkage::Internal);
                    // glob_var.set_initializer(&self.context.i64_type().const_int(0, false));
                    // builder
                    //     .build_store(glob_var.as_pointer_value(), left_val)
                    //     .expect("Failed to store value in the global variable");
                }
                else if op == AstOperation::Print{
                    println!("Generating code for Print");
                    retval = self.context.i64_type().const_int(0, false);
                    let func_gen = FuncGenerator::new(&self.context, &self.module);
                    func_gen.generate_printint();
                    let function = self.module.get_function("shades_printint").expect("failed to retrieve shades_printint");
                    builder.build_call(function, &[left_val.into()], "").expect("failed to call shades_printint");
                }
                else if op == AstOperation::Glue{
                    retval = self.context.i64_type().const_int(0,false);
                }
                else {
                    retval = self.context.i64_type().const_int(0, false);
                    let message = format!("Invalid AST Operation: {:#?}",op);
                    fatal_error(&message, 1);
                }
                return (retval, false);
            } else {
                let value = node.get_val();
                let ret_type = self.context.i64_type();
                match value {
                    Value::Intval(v) => {
                        return (ret_type.const_int(v as u64, false), false);
                    }
                    Value::SlotNumber(n) => {
                        return (ret_type.const_int(n as u64, false), true);
                    }
                }
            }
        } else {
            return (self.context.i64_type().const_int(0, false), false);
        }
    }
}

pub fn build_call_for_cmp<'a: 'b, 'b>(
    module: &Module<'a>,
    builder: &Builder<'b>,
    lhs: BasicValueEnum<'b>,
    rhs: BasicValueEnum<'b>,
    op: &AstOperation,
) -> IntValue<'b> {
    let math_op = ast_op_to_mathop(op);
    let fn_name = FN_OPS.get(&math_op).unwrap();
    let cmp_fn = module.get_function(fn_name).unwrap();
    let retval = builder
        .build_call(cmp_fn, &[lhs.into(), rhs.into()], "cmp_retval")
        .expect("build_call_for_cmp failed")
        .try_as_basic_value()
        .left()
        .expect("build_call_for_cmp failed")
        .into_int_value();
    return retval;
}
