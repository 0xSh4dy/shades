use std::collections::HashMap;

use inkwell::{
    context::Context,
    module::Module,
    values::{BasicValueEnum, IntValue},
    IntPredicate,
};
use lazy_static::lazy_static;

use crate::ast::astnode::AstOperation;

#[derive(PartialEq, Eq, Hash)]
pub enum MathOps {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
}

lazy_static! {
    pub static ref FN_OPS: HashMap<MathOps, &'static str> = {
        let mut map = HashMap::new();
        map.insert(MathOps::Add, "shades_add");
        map.insert(MathOps::Sub, "shades_sub");
        map.insert(MathOps::Mul, "shades_mult");
        map.insert(MathOps::Div, "shades_div");
        map.insert(MathOps::Lt, "shades_lt");
        map.insert(MathOps::Lte, "shades_lte");
        map.insert(MathOps::Gt, "shades_gt");
        map.insert(MathOps::Gte, "shades_gte");
        map.insert(MathOps::Eq, "shades_eq");
        map.insert(MathOps::Neq, "shades_neq");
        return map;
    };
}
pub struct MathCodeGenerator<'a, 'b> {
    context: &'a Context,
    module: &'b Module<'a>,
}

impl<'a, 'b> MathCodeGenerator<'a, 'b> {
    pub fn new(context: &'a Context, module: &'b Module<'a>) -> MathCodeGenerator<'a, 'b> {
        return MathCodeGenerator {
            context: context,
            module: module,
        };
    }
    fn generate_arithmetic_function(&self, op: MathOps,is_cmp:bool) {
        let fn_name = FN_OPS.get(&op).unwrap();
        let function = self.module.get_function(fn_name);

        if function.is_none() {
            let builder = self.context.create_builder();
            let fn_type = if is_cmp{
                let bool_type = self.context.bool_type();
                bool_type.fn_type(&[bool_type.into(),bool_type.into()],false)
            }else{
                let i64_type = self.context.i64_type();
                i64_type.fn_type(&[i64_type.into(),i64_type.into()],false)
            };

            let function = self.module.add_function(fn_name, fn_type, None);
            let basic_block = self.context.append_basic_block(function, "");
            builder.position_at_end(basic_block);

            let first_param = function.get_first_param().unwrap().into_int_value();
            let second_param = function.get_nth_param(1).expect("Failed to fetch second param").into_int_value();
            
            let result = match op {
                MathOps::Add => builder
                    .build_int_add(first_param, second_param, "sum")
                    .unwrap(),
                MathOps::Sub => builder
                    .build_int_sub(first_param, second_param, "diff")
                    .unwrap(),
                MathOps::Mul => builder
                    .build_int_mul(first_param, second_param, "prod")
                    .unwrap(),
                MathOps::Div => builder
                    .build_int_signed_div(first_param, second_param, "div")
                    .unwrap(),
                MathOps::Eq => builder
                    .build_int_compare(IntPredicate::EQ, first_param, second_param, "eq")
                    .unwrap(),
                MathOps::Neq => builder
                    .build_int_compare(IntPredicate::NE, first_param, second_param, "neq")
                    .unwrap(),
                MathOps::Lt => {
                    println!("Right here");
                    builder
                    .build_int_compare(IntPredicate::SLT, first_param, second_param, "lt")
                    .unwrap()},
                MathOps::Lte => builder
                    .build_int_compare(IntPredicate::SLE, first_param, second_param, "lte")
                    .unwrap(),
                MathOps::Gt => builder
                    .build_int_compare(IntPredicate::SGT, first_param, second_param, "gt")
                    .unwrap(),
                MathOps::Gte => builder
                    .build_int_compare(IntPredicate::SGE, first_param, second_param, "gte")
                    .unwrap(),
            };
            builder.build_return(Some(&result)).unwrap();
        }
    }

    pub fn generate_add_int64(&self) {
        self.generate_arithmetic_function(MathOps::Add,false);
    }

    pub fn generate_sub_int64(&self) {
        self.generate_arithmetic_function(MathOps::Sub,false);
    }

    pub fn generate_mul_int64(&self) {
        self.generate_arithmetic_function(MathOps::Mul,false);
    }

    pub fn generate_div_int64(&self) {
        self.generate_arithmetic_function(MathOps::Div,false);
    }

    pub fn generate_cmp_func(&self, op: &AstOperation) {
        let math_op = ast_op_to_mathop(op);
        self.generate_arithmetic_function(math_op,true);
    }
}

pub fn ast_op_to_mathop(op: &AstOperation) -> MathOps {
    match op {
        AstOperation::Add => MathOps::Add,
        AstOperation::Subtract => MathOps::Sub,
        AstOperation::Multiply => MathOps::Mul,
        AstOperation::Divide => MathOps::Div,
        AstOperation::LessThan => MathOps::Lt,
        AstOperation::LessThanEq => MathOps::Lte,
        AstOperation::GreaterThan => MathOps::Gt,
        AstOperation::GreaterThanEq => MathOps::Gte,
        AstOperation::Equal => MathOps::Eq,
        AstOperation::NotEqual => MathOps::Neq,
        _ => panic!("The provided AST operation cannot be converted to a mathematical operation"),
    }
}
