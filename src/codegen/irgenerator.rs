use crate::{
    ast::astnode::{AstNode, AstOperation},
    llvm_wrappers::generators::mathgen::{MathCodeGenerator, MathOps, FN_OPS},
};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, IntValue},
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

    pub fn generate_ir(&self, node_opt: Option<&Box<AstNode>>) -> IntValue {
        if let Some(node) = node_opt {
            if !node.is_leaf_node() {
                let math_gen = MathCodeGenerator::new(&self.context, &self.module);
                let op = node.get_op();
                let left_val = self.generate_ir(node.get_left_child());
                let right_val = self.generate_ir(node.get_right_child());
                let builder = self.context.create_builder();
                let first_bb = self.parent_func.get_first_basic_block().unwrap();
                builder.position_at_end(first_bb);
                let retval: IntValue;

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
                } else {
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
                }
                return retval;
            } else {
                let value = node.get_intval();
                let ret_type = self.context.i64_type();
                return ret_type.const_int(value as u64, false);
            }
        } else {
            return self.context.i64_type().const_int(0, false);
        }
    }
}
