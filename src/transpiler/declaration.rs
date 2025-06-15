use crate::{
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, ast::Type},
    transpiler::utils::{map_type, transpile_input_var},
};

pub fn transpile_mutable_decl(
    name: &str,
    typ: &Option<Type>,
    value: &Expr,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    if let Some(args) = is_input_expr(value) {
        return transpile_input_var(name, &Type::Metn, args, ctx, true);
    }

    let value_code = transpile_expr(value, ctx)?;
    let inferred_type = match typ {
        Some(t) => t.clone(),
        None => ctx
            .symbol_types
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Tip kontekstdə tapılmadı: '{}'", name))?,
    };

    let decl = match &inferred_type {
        Type::Metn => {
            ctx.needs_allocator = true;
            format!(
                "var {}: []u8 = try allocator.dupe(u8, {});",
                name, value_code
            )
        }
        Type::Siyahi(inner) => {
            if let Expr::List(items) = value {
                let items_code: Result<Vec<_>, _> =
                    items.iter().map(|i| transpile_expr(i, ctx)).collect();
                let items_str = items_code?.join(", ");

                ctx.needs_allocator = true;
                ctx.cleanup_statements.push(format!("{}.deinit();", name));

                let inner_type = map_type(inner, false);
                format!(
                    r#"var {name} = try std.ArrayList({inner_type}).initCapacity(allocator, {cap});
try {name}.appendSlice(&[_]{inner_type}{{ {items_str} }});"#,
                    name = name,
                    cap = items.len(),
                    inner_type = inner_type,
                    items_str = items_str
                )
            } else {
                return Err("Siyahı tipli dəyişən üçün dəyər siyahı olmalıdır.".to_string());
            }
        }
        _ => {
            format!(
                "var {}: {} = {};",
                name,
                map_type(&inferred_type, false),
                value_code
            )
        }
    };

    Ok(decl)
}

pub fn transpile_constant_decl(
    name: &str,
    typ: &Option<Type>,
    value: &Expr,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    if let Some(args) = is_input_expr(value) {
        // `giriş` funksiyasından daxil olan dəyərlər üçün
        return transpile_input_var(name, &Type::Metn, args, ctx, false);
    }

    let inferred_type = match typ {
        Some(t) => t.clone(),
        None => ctx
            .symbol_types
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Tip kontekstdə tapılmadı: '{}'", name))?,
    };

    // 🚀 Əgər value "böl(...)" funksiyasıdırsa
    if let Expr::MethodCall {
        target,
        method,
        args,
    } = value
    {
        if method == "böl" {
            println!("Böl funksiyası istifadə edildi");
            ctx.used_split_n_fn = true;
            let target_code = transpile_expr(target, ctx)?;
            let mut delimiter_code = transpile_expr(&args[0], ctx)?;
            let var_result = format!("result_{}", name);
            delimiter_code = delimiter_code.replace("\"", "\'");

            return Ok(format!(
                r#"const {result} = splitN({target}, {delim}, 32);
    const {name} = {result}.parts[0..{result}.len];"#,
                result = var_result,
                target = target_code,
                delim = delimiter_code,
                name = name
            ));
        }
    }

    // Siyahı tipli konstant dəyərlər
    if let Expr::List(items) = value {
        let items_code: Result<Vec<String>, String> =
            items.iter().map(|item| transpile_expr(item, ctx)).collect();
        let items_str = items_code?.join(", ");

        let actual_type = typ.clone().unwrap_or_else(|| inferred_type.clone());

        if let Type::Siyahi(inner_type) = actual_type {
            if items.is_empty() {
                return Ok(format!(
                    "const {} = &[_]{}{{}};",
                    name,
                    map_type(&*inner_type, true)
                ));
            }

            return Ok(format!(
                "const {}: {} = &[_]{}{{ {} }};",
                name,
                map_type(&Type::Siyahi(inner_type.clone()), true),
                map_type(&*inner_type, true),
                items_str
            ));
        }

        if items.is_empty() && typ.is_none() {
            return Ok(format!("const {} = &{{}};", name));
        }
    }

    // Digər konstant dəyərlər üçün
    let value_code = transpile_expr(value, ctx)?;
    Ok(format!(
        "const {}: {} = {};",
        name,
        map_type(&inferred_type, true),
        value_code
    ))
}

fn is_input_expr(expr: &Expr) -> Option<&[Expr]> {
    match expr {
        Expr::BuiltInCall { func, args }
            if matches!(func, crate::parser::ast::BuiltInFunction::Input) =>
        {
            Some(args)
        }
        Expr::FunctionCall { name, args } if name == "giriş" => Some(args),
        _ => None,
    }
}
