use crate::{
    ast::{astnode::{create_leaf_node, AstNode, AstOperation, Value}, asttree::build_ast},
    codegen::irgenerator::IrGenerator,
    lexer::{symbols::symtab::find_symbol, tokens::{TokenList, TokenTypes, TokenValue}},
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
        tokens.next();
        if let Some(cur_tok) = cur_token_opt{
            if let TokenValue::String(val) = cur_tok.get_value(){
                let find_stats = find_symbol(&val);
                if let Some(idx) = find_stats{
                    let right = create_leaf_node(AstOperation::Lvident, Value::SlotNumber(idx));
                    let left = build_ast(tokens, 0);
                    let root = AstNode::create(AstOperation::Assign, left, Some(right), 0);
                    self.generate_ir(Some(root).as_ref());
                }
                else{
                    fatal_error(&format!("Undeclared variable {}",val), 1)
                }
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
