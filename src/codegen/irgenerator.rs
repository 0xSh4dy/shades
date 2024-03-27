use crate::{
    ast::astnode::{AstNode, AstOperation, Value},
    lexer::symbols::symtab::get_corresp_symtab,
    llvm_wrappers::generators::{
        funcgen::FuncGenerator,
        mathgen::{self, ast_op_to_mathop, MathCodeGenerator, MathOps, FN_OPS},
    },
    utils::errors::fatal_error,
};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    types::IntType,
    values::{BasicValue, BasicValueEnum, FunctionValue, IntValue},
    IntPredicate,
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

    pub fn generate_ir(&self, node_opt: Option<&Box<AstNode>>) {
        let mut builder = self.context.create_builder();
        let entry_block = self.context.append_basic_block(self.parent_func, "entry");
        builder.position_at_end(entry_block);

        let math_gen = mathgen::MathCodeGenerator::new(self.context, self.module);
        math_gen.generate_add_int64();

        let first_bb = self
            .parent_func
            .get_first_basic_block()
            .expect("failed to get the first basic block");
        self.generate_code(first_bb, node_opt);
        let last_bb = self
            .parent_func
            .get_last_basic_block()
            .expect("failed to retrieve last basic block");
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

    fn generate_code(
        &self,
        block: BasicBlock,
        node_opt: Option<&Box<AstNode>>,
    ) -> (IntValue, bool) {
        if let Some(node) = node_opt {
            if !node.is_leaf_node() {
                let op = node.get_op();
                let builder = self.context.create_builder();
                let retval: IntValue;

                if op == AstOperation::If {
                    retval = self.context.i64_type().const_int(0, false);
                    // Get the left conditional tree
                    let cond_tree = node.get_left_child();
                    let then_tree = node.get_mid_child();
                    let else_tree = node.get_right_child();

                    let cond = self.generate_code(block, cond_tree).0;

                    let then_block = self.context.append_basic_block(self.parent_func, "then");
                    let else_block = self.context.append_basic_block(self.parent_func, "else");
                    let ifcont_block = self.context.append_basic_block(self.parent_func, "ifcont");

                    // Build the conditional brancg
                    builder.position_at_end(block);
                    builder
                        .build_conditional_branch(cond, then_block, else_block)
                        .expect("failed to build conditional branch");

                    // Generate code for then block
                    builder.position_at_end(then_block);
                    self.generate_code(then_block, then_tree);
                    builder
                        .build_unconditional_branch(ifcont_block)
                        .expect("failed to build unconditional branch");

                    // Generate code for else block
                    builder.position_at_end(else_block);
                    self.generate_code(else_block, else_tree);
                    builder
                        .build_unconditional_branch(ifcont_block)
                        .expect("failed to build unconditional branch");

                    return (retval, false);
                } else if op == AstOperation::While {
                    retval = self.context.i64_type().const_int(0, false);

                    let loop_cond_block = self
                        .context
                        .append_basic_block(self.parent_func, "loop_cond");
                    let loop_inner_block = self
                        .context
                        .append_basic_block(self.parent_func, "loop_inner");
                    let loop_end_block = self
                        .context
                        .append_basic_block(self.parent_func, "loop_end");

                    let cond_tree = node.get_left_child();
                    let body_tree = node.get_right_child();

                    let cond_result = self.generate_code(loop_cond_block, cond_tree).0;
                    self.generate_code(loop_inner_block, body_tree);

                    builder.position_at_end(block);
                    builder
                        .build_unconditional_branch(loop_cond_block)
                        .expect("failed to build unconditional branch");

                    builder.position_at_end(loop_cond_block);
                    builder
                        .build_conditional_branch(cond_result, loop_inner_block, loop_end_block)
                        .expect("failed to build conditional branch");
                    builder.position_at_end(loop_inner_block);
                    builder
                        .build_unconditional_branch(loop_cond_block)
                        .expect("failed to build conditional branch");
                    return (retval, false);
                }

                builder.position_at_end(block);

                let (left_val, slot_ret_left) = self.generate_code(block, node.get_left_child());
                let (right_val, slot_ret_right) = self.generate_code(block, node.get_right_child());

                if op == AstOperation::Assign {
                    retval = self.context.i64_type().const_int(0, false);

                    let slot_num = right_val.get_sign_extended_constant().unwrap();
                    let name = get_corresp_symtab(slot_num as usize).get_name();

                    let glob_var_opt = self.module.get_global(&name);
                    if let Some(glob_var) = glob_var_opt {
                        builder
                            .build_store(glob_var.as_pointer_value(), left_val)
                            .expect("build_store failed");
                    } else {
                        let glob_var =
                            self.module
                                .add_global(self.context.i64_type(), None, name.as_str());
                        glob_var.set_linkage(Linkage::Internal);
                        glob_var.set_initializer(&self.context.i64_type().const_int(0, false));
                        builder
                            .build_store(glob_var.as_pointer_value(), left_val)
                            .expect("build_store failed");
                    }
                    return (retval,false);
                }
                let left_val = if slot_ret_left {
                    let symtab = get_corresp_symtab(
                        left_val
                            .get_sign_extended_constant()
                            .expect("get_sign_extended_constant failed")
                            as usize,
                    );
                    println!("{}", symtab.get_name());
                    let glob_value = self.module.get_global(&symtab.get_name()).expect(&format!(
                        "Variable {} might not have been initialized",
                        symtab.get_name()
                    ));
                    let ptr_value = glob_value.as_pointer_value();
                    let ptr_type = self.context.i64_type();
                    let loaded = builder
                        .build_load(ptr_type, ptr_value, &symtab.get_name())
                        .expect("build_load failed");
                    loaded
                } else {
                    left_val.as_basic_value_enum()
                };

                let right_val = if slot_ret_right {
                    let symtab = get_corresp_symtab(
                        right_val
                            .get_sign_extended_constant()
                            .expect("get_sign_extended_constant failed")
                            as usize,
                    );
                    println!("{}", symtab.get_name());

                    let glob_value = self.module.get_global(&symtab.get_name()).expect(&format!(
                        "Variable {} might not have been initialized",
                        symtab.get_name()
                    ));
                    let ptr_value = glob_value.as_pointer_value();
                    let ptr_type = self.context.i64_type();
                    let loaded = builder
                        .build_load(ptr_type, ptr_value, &symtab.get_name())
                        .expect("build_load failed");
                    loaded
                } else {
                    right_val.as_basic_value_enum()
                };
                if op == AstOperation::Add {
                    println!("left_val: {:#?} {}", left_val, slot_ret_left);
                    println!("right_val: {:#?} {}", right_val, slot_ret_right);

                    retval = builder
                        .build_int_add(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "vadd",
                        )
                        .expect("build_int_add failed");
                } else if op == AstOperation::Subtract {
                    retval = builder
                        .build_int_sub(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "vsub",
                        )
                        .expect("build_int_sub failed");
                } else if op == AstOperation::Multiply {
                    retval = builder
                        .build_int_sub(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "vmul",
                        )
                        .expect("build_int_mul failed");
                } else if op == AstOperation::Divide {
                    retval = builder
                        .build_int_sub(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "vdiv",
                        )
                        .expect("build_int_div failed");
                } else if op == AstOperation::LessThan {
                    retval = builder
                        .build_int_compare(
                            IntPredicate::SLT,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "slt",
                        )
                        .expect("build_int_compare failed");
                } else if op == AstOperation::GreaterThan {
                    retval = builder
                        .build_int_compare(
                            IntPredicate::SGT,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "sgt",
                        )
                        .expect("build_int_compare failed");
                } else if op == AstOperation::GreaterThanEq {
                    retval = builder
                        .build_int_compare(
                            IntPredicate::SGE,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "sgeq",
                        )
                        .expect("build_int_compare sgeq failed");
                } else if op == AstOperation::LessThanEq {
                    retval = builder
                        .build_int_compare(
                            IntPredicate::SLE,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "sleq",
                        )
                        .expect("build_int_compare sleq failed");
                } else if op == AstOperation::Equal {
                    retval = builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "eq",
                        )
                        .expect("build_int_compare eq failed");
                } else if op == AstOperation::NotEqual {
                    retval = builder
                        .build_int_compare(
                            IntPredicate::NE,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "eq",
                        )
                        .expect("build_int_compare neq failed");
                } else if op == AstOperation::Print {
                    retval = self.context.i64_type().const_int(0, false);
                    let func_gen = FuncGenerator::new(&self.context, &self.module);
                    func_gen.generate_printint();
                    let function = self
                        .module
                        .get_function("shades_printint")
                        .expect("failed to retrieve shades_printint");
                    builder
                        .build_call(function, &[left_val.into()], "")
                        .expect("failed to call shades_printint");
                } else if op == AstOperation::Glue {
                    retval = self.context.i64_type().const_int(0, false);
                } else {
                    retval = self.context.i64_type().const_int(0, false);
                    let message = format!("Invalid AST Operation: {:#?}", op);
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