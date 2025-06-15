use crate::{
    builtin::match_builtin,
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, ast::Type},
    transpiler::utils::map_type,
};

pub fn transpile_function_call(
    name: &str,
    args: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    if let Some(func) = match_builtin(name) {
        transpile_expr(
            &Expr::BuiltInCall {
                func,
                args: args.to_vec(),
            },
            ctx,
        )
    } else {
        // Her argüman bir değişkense, onun tipini context'e ekleyerek analiz et
        for arg in args {
            if let Expr::VariableRef(var_name) = arg {
                // Zaten ctx.symbol_types içinde varsa tekrar eklemeye gerek yok
                if !ctx.symbol_types.contains_key(var_name) {
                    // Basit örnek: string sabitse, "mətn" tipi atayabiliriz
                    ctx.symbol_types.insert(var_name.clone(), Type::Metn);
                }
            }
        }

        let args_code: Result<Vec<String>, String> =
            args.iter().map(|arg| transpile_expr(arg, ctx)).collect();
        let args_code = args_code?;
        Ok(format!("{}({})", name, args_code.join(", ")))
    }
}

pub fn transpile_function_def(
    name: &str,
    params: &[(String, Type, bool)],
    body: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    let params_str: Vec<String> = params
        .iter()
        .map(|(param_name, declared_type, is_mutable)| {
            let rust_type = map_type(declared_type, !is_mutable);
            format!("{}: {}", param_name, rust_type)
        })
        .collect();

    // Blokun daxilindəki ifadələri transpile et
    let mut body_lines = Vec::new();
    for expr in body {
        let line = transpile_expr(expr, ctx)?;
        body_lines.push(format!("    {}", line));
    }

    Ok(format!(
        "fn {}({}) {{\n{}\n}}",
        name,
        params_str.join(", "),
        body_lines.join("\n")
    ))
}
