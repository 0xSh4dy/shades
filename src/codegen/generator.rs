use inkwell::context::Context;

use super::codeinterp::interpret_ast;
use super::irgenerator::IrGenerator;
use crate::ast::asttree::build_ast;
use crate::lexer::keywords::match_keyword;
use crate::lexer::tokens::{TokenList, TokenTypes};
use crate::llvm_wrappers::generators::funcgen::FuncGenerator;

pub fn generate_code(tokens: &mut TokenList) {
    let context = Context::create();
    let module = context.create_module("main");
    let func_generator = FuncGenerator::new(&context, &module);
    func_generator.generate_c_main_function();
    func_generator.generate_shades_main_function();
    loop {
        // Find whether the first token is print or not
        match_keyword(tokens.next().unwrap().get_type());
        let root = build_ast(tokens, 0);
        let val = interpret_ast(root.as_ref());
        println!("Result: {}", val);
        println!("Generating Assembly..............");
        // let mut x64regs = X64Registers::new();
        // x64regs.func_preamble();
        unsafe {
            let ir_generator = IrGenerator::new(&context, module.clone());
            ir_generator.generate_ir(root.as_ref());
            // The main function is modified over here
            // ir_generator.generate_ir(root.as_ref());
            // ir_generator.perform_cleanup();
        }
        if let Some(tok) = tokens.peek() {
            if tok.get_type() == TokenTypes::T_SEMICOLON {
                tokens.next();
                if let Some(tok) = tokens.peek() {
                    if tok.get_type() == TokenTypes::T_EOF {
                        break;
                    }
                }
            }
        }
        // generate_asm(root.as_ref(), &mut x64regs);
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
        // x64regs.func_postamble();
    }
}
