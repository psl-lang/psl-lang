use crate::ast::Program;

use self::context::CodegenContext;

mod context;
mod impls;
mod visitor;

pub fn generate_codes(ast: Program) -> String {
    let mut ctx = CodegenContext::new();
    
    ctx.visit(ast)
}
