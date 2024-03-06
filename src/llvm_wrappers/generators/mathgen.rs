use std::collections::HashMap;

use inkwell::{context::Context, module::Module};
use lazy_static::lazy_static;

#[derive(PartialEq,Eq,Hash)]
enum MathOps{
    Add,
    Sub,
    Mul,
    Div
}

lazy_static!{
    static ref FN_OPS : HashMap<MathOps,&'static str> = {
        let mut map = HashMap::new();
        map.insert(MathOps::Add,"shades_add");
        map.insert(MathOps::Sub,"shades_sub");
        map.insert(MathOps::Mul,"shades_mult");
        map.insert(MathOps::Div,"shades_div");
        return map;
    };
}
pub struct MathCodeGenerator<'a,'b>{
    context:&'a Context,
    module:&'b Module<'a>
}

impl <'a,'b>MathCodeGenerator<'a,'b>{
    pub fn new(context:&'a Context,module:&'b Module<'a>)->MathCodeGenerator<'a,'b>{
        return MathCodeGenerator{
            context:context,
            module:module,
        };
    }
    fn generate_arithmetic_function(&self, op: MathOps) {
        let fn_name = FN_OPS.get(&op).unwrap();
        let function = self.module.get_function(fn_name);
    
        if function.is_none() {
            let builder = self.context.create_builder();
            let i64_type = self.context.i64_type();
            let fn_type = i64_type.fn_type(&[i64_type.into(),i64_type.into()],false);
            
            let function = self.module.add_function(fn_name, fn_type, None);
            let basic_block = self.context.append_basic_block(function, "");
            builder.position_at_end(basic_block);
            
            let first_param = function.get_first_param().unwrap().into_int_value();
            let second_param = function.get_nth_param(1).unwrap().into_int_value();
            
            let result = match op {
                MathOps::Add => builder.build_int_add(first_param, second_param, "sum").unwrap(),
                MathOps::Sub => builder.build_int_sub(first_param, second_param, "diff").unwrap(),
                MathOps::Mul => builder.build_int_mul(first_param,second_param,"prod").unwrap(),
                MathOps::Div => builder.build_int_signed_div(first_param,second_param,"div").unwrap(),
            };
            
            builder.build_return(Some(&result)).unwrap();
        }
    }

    pub fn generate_add_int64(&self) {
        self.generate_arithmetic_function(MathOps::Add);
    }

    pub fn generate_sub_int64(&self){
        self.generate_arithmetic_function(MathOps::Sub);
    }

    pub fn generate_mul_int64(&self){
        self.generate_arithmetic_function(MathOps::Mul);
    }

    pub fn generate_div_int64(&self){
        self.generate_arithmetic_function(MathOps::Div);
    }
}

