use inkwell::{context::Context, module::Module};

pub struct FuncGenerator<'a,'b>{
    context:&'a Context,
    module:&'b Module<'a>  
}

#[allow(dead_code)]
impl <'a,'b>FuncGenerator<'a,'b>{
    pub fn new(context:&'a Context,module:&'b Module<'a>)->FuncGenerator<'a,'b>{
        return FuncGenerator{
            context:context,
            module:module
        };
    }

    pub fn generate_c_main_function(&self){
        let builder = self.context.create_builder();
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[],false);
        let function = self.module.add_function("main",fn_type,None);
        let basic_block = self.context.append_basic_block(function,"");
        let const_zero = i32_type.const_zero();
        let shades_main_fn = self.module.get_function("shades_main").unwrap();
        builder.position_at_end(basic_block);
        builder.build_call(shades_main_fn,&[],"").unwrap();
        builder.build_return(Some(&const_zero)).unwrap();
    }

    fn generate_default_void_fn(&self,fn_name:&str){
        let void_type = self.context.void_type();
        let func_type = void_type.fn_type(&[],false);
        let function = self.module.add_function(fn_name,func_type,None);
        let basic_block = self.context.append_basic_block(function,"");
        let builder = self.context.create_builder();
        builder.position_at_end(basic_block);
        builder.build_return(None).unwrap();
    }
}
