use super::asmgen::generate_asm;
use super::codeinterp::interpret_ast;
use crate::ast::asttree::build_ast;
use crate::codegenerator::x64::registers::X64Registers;
use crate::lexer::keywords::match_keyword;
use crate::lexer::tokens::{TokenList, TokenTypes};

pub fn generate_code(tokens: &mut TokenList) {
    loop {
        // Find whether the first token is print or not
        match_keyword(tokens.next().unwrap().get_type());
        let root = build_ast(tokens, 0);
        let val = interpret_ast(root.as_ref());
        println!("Result: {}", val);
        println!("Generating Assembly..............");
        let mut x64regs = X64Registers::new();
        // x64regs.func_preamble();
        generate_asm(root.as_ref(), &mut x64regs);
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
        // x64regs.func_postamble();
    }
}
