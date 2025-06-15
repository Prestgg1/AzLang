use crate::context::TranspileContext;
use crate::parser::Expr;
use crate::parser::ast::Type;
use crate::transpiler::expr::transpile_expr;

pub fn transpile_loop(
    var_name: &str,
    iterable: &Expr,
    body: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    let iterable_name_opt = if let Expr::VariableRef(name) = iterable {
        Some(name)
    } else {
        None
    };
    let is_mutable = iterable_name_opt
        .map(|name| ctx.mutable_symbols.contains(name))
        .unwrap_or(false);
    let iterable_code = transpile_expr(iterable, ctx)?;
    let body_code = body
        .iter()
        .map(|stmt| transpile_expr(stmt, ctx))
        .collect::<Result<Vec<String>, String>>()?;

    if is_mutable {
        Ok(format!(
            "for ({}.items ) |{}| {{\n{}\n}}",
            iterable_code,
            var_name,
            body_code.join("\n")
        ))
    } else {
        Ok(format!(
            "for ({} ) |{}| {{\n{}\n}}",
            iterable_code,
            var_name,
            body_code.join("\n")
        ))
    }
}
