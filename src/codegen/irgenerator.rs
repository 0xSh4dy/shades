use crate::{
    ast::astnode::{AstNode, AstOperation, Value},
    lexer::symbols::symtab::get_corresp_symtab,
    llvm_wrappers::generators::mathgen::{ast_op_to_mathop, MathCodeGenerator, MathOps, FN_OPS},
    utils::errors::fatal_error,
};
use inkwell::{
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

    // The bool indicates if a slot number was returned
    pub fn generate_ir(&self, node_opt: Option<&Box<AstNode>>) -> (IntValue, bool) {
        if let Some(node) = node_opt {
            if !node.is_leaf_node() {
                let math_gen = MathCodeGenerator::new(&self.context, &self.module);
                let op = node.get_op();
                let (left_val, slot_ret_left) = self.generate_ir(node.get_left_child());
                let (right_val, slot_ret_right) = self.generate_ir(node.get_right_child());
                let builder = self.context.create_builder();
                let first_bb = self.parent_func.get_first_basic_block().unwrap();
                builder.position_at_end(first_bb);
                let retval: IntValue;

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
                    let glob_var =
                        self.module
                            .add_global(self.context.i64_type(), None, name.as_str());
                    glob_var.set_linkage(Linkage::Internal);
                    glob_var.set_initializer(&self.context.i64_type().const_int(0, false));
                    builder
                        .build_store(glob_var.as_pointer_value(), left_val)
                        .expect("Failed to store value in the global variable");
                } else {
                    retval = self.context.i64_type().const_int(0, false);
                    fatal_error("Invalid AST operation", 1);
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
