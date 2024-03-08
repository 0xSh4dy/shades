use super::irgenerator::IrGenerator;
use crate::ast::asttree::build_ast;
use crate::lexer::keywords::matcher::{match_and_generate};
use crate::lexer::tokens::{TokenList, TokenTypes};
use crate::llvm_wrappers::generators::funcgen::FuncGenerator;
use inkwell::context::Context;

pub fn generate_code(tokens: &mut TokenList) {
    let context = Context::create();
    let module = context.create_module("main");
  
    // Generate the shades_main function
    let void_type = context.void_type();
    let func_type = void_type.fn_type(&[], false);
    let function = module.add_function("shades_main", func_type, None);
    let basic_block = context.append_basic_block(function, "");

    // Generate the main function
    let func_generator = FuncGenerator::new(&context, &module);
    func_generator.generate_c_main_function();

    // Loop line-by-line
    // loop {
        // Find whether the first token is print or not
        // match_keyword(tokens.next().unwrap().get_type());
        match_and_generate(tokens,&context,&module);
        // if tokens.peek().unwrap().get_type() == TokenTypes::T_EOF{
        //     break;
        // }
        // let root = build_ast(tokens, 0);
        // let ir_generator = IrGenerator::new(&context, &module, "shades_main");
        // ir_generator.generate_ir(root.as_ref());
        // if let Some(tok) = tokens.peek() {
        //     if tok.get_type() == TokenTypes::T_SEMICOLON {
        //         tokens.next();
        //         if let Some(tok) = tokens.peek() {
        //             if tok.get_type() == TokenTypes::T_EOF {
        //                 break;
        //             }
        //         }
        //     }
        // }
    // }

    // return void for shades_main
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(None).unwrap();
    module.print_to_file("/tmp/main.ll").unwrap();
}
