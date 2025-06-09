use super::utils::map_type;
use crate::parser::{Expr, ast::BuiltInFunction};

use std::collections::HashSet;

pub struct TranspileContext {
    pub imports: HashSet<String>,
}

impl TranspileContext {
    pub fn new() -> Self {
        Self {
            imports: HashSet::new(),
        }
    }

    pub fn add_import(&mut self, import: &str) -> Option<String> {
        if self.imports.contains(import) {
            None
        } else {
            self.imports.insert(import.to_string());
            Some(import.to_string())
        }
    }
}

pub fn transpile_expr(expr: &Expr, ctx: &mut TranspileContext) -> Result<String, String> {
    match expr {
        Expr::BuiltInCall { func, args } => match func {
            BuiltInFunction::Print => {
                if args.len() != 1 {
                    return Err("print funksiyası yalnız 1 arqument qəbul edir".to_string());
                }

                let arg_code = transpile_expr(&args[0], ctx)?;

                let format_str = if matches!(&args[0], Expr::String(_)) {
                    "{}"
                } else if let Expr::FunctionCall { name, .. } = &args[0] {
                    if name == "number" { "{}" } else { "{:?}" }
                } else {
                    "{:?}"
                };

                Ok(format!("println!(\"{}\", {});", format_str, arg_code))
            }

            BuiltInFunction::Number => {
                if args.len() != 1 {
                    return Err("number funksiyası yalnız 1 arqument qəbul edir".to_string());
                }

                match &args[0] {
                    Expr::String(s) => {
                        if s.chars().all(|c| c.is_ascii_digit()) {
                            Ok(s.clone())
                        } else {
                            Err(format!(
                                "number funksiyasına yalnız rəqəmlər verilməlidir: {:?}",
                                s
                            ))
                        }
                    }
                    Expr::FunctionCall { .. } | Expr::VariableRef(_) => {
                        let inner = transpile_expr(&args[0], ctx)?;
                        Ok(format!("{}.parse::<i32>().unwrap()", inner))
                    }
                    _ => Err(
                        "number funksiyasına yalnız string və ya dəyişən verilməlidir".to_string(),
                    ),
                }
            }

            BuiltInFunction::Input => {
                if args.len() != 1 {
                    return Err("input funksiyası yalnız 1 arqument qəbul edir".to_string());
                }

                let prompt = transpile_expr(&args[0], ctx)?;

                ctx.add_import("use std::io::Write;");

                Ok(format!(
                    r#"{{
                print!("{}", {});
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            }}"#,
                    "{}", prompt
                ))
            }

            BuiltInFunction::Len => {
                if args.len() != 1 {
                    return Err("len funksiyası yalnız 1 arqument qəbul edir".to_string());
                }

                let arg_code = transpile_expr(&args[0], ctx)?;
                Ok(format!("{}.len()", arg_code))
            }
        },
        Expr::Return(e) => {
            let expr_code = transpile_expr(e, ctx)?;
            Ok(format!("return {}", expr_code))
        }
        Expr::FunctionCall { name, args } => {
            if let Some(func) = match_builtin(name) {
                transpile_expr(
                    &Expr::BuiltInCall {
                        func,
                        args: args.clone(),
                    },
                    ctx,
                )
            } else {
                let args_code: Result<Vec<String>, String> =
                    args.iter().map(|arg| transpile_expr(arg, ctx)).collect();
                let args_code = args_code?;
                Ok(format!("{}({})", name, args_code.join(", ")))
            }
        }

        Expr::MutableDecl { name, typ, value } => {
            let value_code = transpile_expr(value, ctx)?;
            Ok(format!(
                "let mut {}: {} = {};",
                name,
                map_type(typ, false),
                value_code
            ))
        }

        Expr::ConstantDecl { name, typ, value } => {
            let is_array = typ.starts_with("siyahı<");

            // Xüsusi: Sabit listləri &[...] kimi yarat
            if is_array {
                if let Expr::List(items) = &**value {
                    let items_code: Result<Vec<String>, String> =
                        items.iter().map(|item| transpile_expr(item, ctx)).collect();
                    let items_str = items_code?.join(", ");

                    // Sabit list üçün &[...] sintaksisi
                    return Ok(format!(
                        "const {}: {} = &[{}];",
                        name,
                        map_type(typ, true),
                        items_str
                    ));
                }
            }

            // Normal sabitlər üçün
            let value_code = transpile_expr(value, ctx)?;
            Ok(format!(
                "const {}: {} = {};",
                name,
                map_type(typ, true),
                value_code
            ))
        }

        Expr::List(items) => {
            let items_code: Result<Vec<String>, String> =
                items.iter().map(|item| transpile_expr(item, ctx)).collect();
            let items_str = items_code?.join(", ");
            Ok(format!("vec![{}]", items_str))
        }

        Expr::String(s) => Ok(format!("\"{}\"", s)),

        Expr::VariableRef(name) => Ok(name.clone()),

        Expr::FunctionDef { name, params, body } => {
            let params_str: Vec<String> = params
                .iter()
                .map(|(param_name, param_type)| {
                    let rust_type = map_type(param_type, true);
                    format!("{}: {}", param_name, rust_type)
                })
                .collect();

            let body_str = body
                .iter()
                .map(|expr| transpile_expr(expr, ctx))
                .collect::<Result<Vec<_>, _>>()?
                .join("\n");

            Ok(format!(
                "fn {}({}) {{\n{}\n}}",
                name,
                params_str.join(", "),
                body_str
            ))
        }
    }
}

fn match_builtin(name: &str) -> Option<BuiltInFunction> {
    match name {
        "çap" | "print" => Some(BuiltInFunction::Print),
        "giriş" | "input" => Some(BuiltInFunction::Input),
        "uzunluq" | "len" => Some(BuiltInFunction::Len),
        "ədəd" | "number" => Some(BuiltInFunction::Number),
        _ => None,
    }
}
