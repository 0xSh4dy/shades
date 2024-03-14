use crate::lexer::scanner::tokens::TokenList;
use crate::llvm_wrappers::generators::funcgen::FuncGenerator;
use inkwell::context::Context;
use crate::lexer::scanner::mainloop::handle_compound_statement;

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

    // Match a compound statement
    handle_compound_statement(tokens, &context, &module);

    // return void for shades_main
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(None).unwrap();
    module.print_to_file("/tmp/main.ll").unwrap();
}
