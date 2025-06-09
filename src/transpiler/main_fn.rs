use super::expr::transpile_expr;
use crate::{expr::TranspileContext, parser::Program};

pub fn generate_main_fn(program: &Program) -> Result<String, String> {
    let mut ctx = TranspileContext::new();
    let mut body = String::new();

    for expr in &program.expressions {
        let line = transpile_expr(expr, &mut ctx)?;
        body.push_str("    ");
        body.push_str(&line);
        body.push_str("\n");
    }

    let mut rust_code = String::new();

    for import in &ctx.imports {
        rust_code.push_str(import);
        rust_code.push_str("\n");
    }

    rust_code.push_str("\nfn main() {\n");
    rust_code.push_str(&body);
    rust_code.push_str("}\n");

    Ok(rust_code)
}
