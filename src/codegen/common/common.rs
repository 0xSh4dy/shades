use crate::{
    ast::asttree::build_ast,
    codegen::irgenerator::IrGenerator,
    lexer::tokens::{TokenList, TokenTypes},
    utils::errors::fatal_error,
};

impl<'a, 'b> IrGenerator<'a, 'b> {
    pub fn handle_print_decl(&self, tokens: &mut TokenList) {
        let root = build_ast(tokens, 0);
        self.generate_ir(root.as_ref());
    }
    pub fn handle_assignment(&self, tokens: &mut TokenList) {
        let cur_token_opt = tokens.next();
        
        // Validate the syntax
        if let Some(next_token) = tokens.peek() {
            if next_token.get_type() != TokenTypes::T_EQUAL {
                fatal_error("Syntax error in assignment", 1);
            }
        }
        
        // if let Some(cur_token) = cur_token_opt{
        //     if cur_token.get_type() == TokenTypes::T_EQUAL{
        //         let root = build_ast(tokens,0);
        //         self.generate_ir(root.as_ref());
        //         return
        //     }
        // }
    }
}
