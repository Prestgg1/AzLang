use crate::parser::Statement;
use crate::print_utils::format_print;
use crate::warning::{Warning, warn};
use regex::Regex;
use std::collections::HashSet;

pub fn transpile(statements: &[Statement]) -> String {
    let mut rust_code = String::new();
    let mut functions = Vec::new();
    let mut main_body = Vec::new();

    let mut mutable_vars: HashSet<String> = HashSet::new(); // bütün `let mut` dəyişənlərini saxlayırıq
    let mut used_as_assignment: HashSet<String> = HashSet::new(); // dəyişdirilən dəyişənləri saxlayırıq

    for statement in statements {
        match statement {
            Statement::Print(content) => {
                main_body.push(format!("    {};", format_print(content)));

                // Dəyişən adlarının dəyişdirildiyini analiz etmək üçün sadə regex
                for var in &mutable_vars {
                    let pattern = format!(r"\b{}\s*=", var);
                    let re = Regex::new(&pattern).unwrap();
                    if re.is_match(content) {
                        used_as_assignment.insert(var.clone());
                    }
                }
            }

            Statement::FunctionDef { name, params, body } => {
                let mut fn_code = String::new();

                let params_str = params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t.to_rust_type()))
                    .collect::<Vec<_>>()
                    .join(", ");

                fn_code.push_str(&format!("fn {}({}) {{\n", name, params_str));
                let mut used_vars = HashSet::new();

                for stmt in body {
                    if let Statement::Print(content) = stmt {
                        fn_code.push_str(&format!("    {};\n", format_print(content)));

                        for (param_name, _) in params {
                            let pattern = format!(r"\b{}\b", param_name);
                            let re = Regex::new(&pattern).unwrap();
                            if re.is_match(content) {
                                used_vars.insert(param_name.to_string());
                            }
                        }
                    }
                }

                for (name, _) in params {
                    if !used_vars.contains(name) {
                        warn(Warning::UnusedParam(name.clone()));
                    }
                }

                fn_code.push_str("}\n\n");
                functions.push(fn_code);
            }

            Statement::FunctionCall { name, args } => {
                main_body.push(format!("    {}({});", name, args.join(", ")));
            }

            Statement::VariableDecl {
                mutable,
                name,
                var_type,
                value,
            } => {
                let rust_type = var_type.to_rust_type();
                if *mutable {
                    mutable_vars.insert(name.clone());
                    main_body.push(format!("    let mut {}: {} = {};", name, rust_type, value));
                } else {
                    main_body.push(format!("    const {}: {} = {};", name, rust_type, value));
                }
            }
        }
    }

    for var in &mutable_vars {
        if !used_as_assignment.contains(var) {
            warn(Warning::MutableButNeverChanged(var.clone()));
        }
    }

    rust_code.push_str(&functions.join(""));
    rust_code.push_str("fn main() {\n");
    rust_code.push_str(&main_body.join("\n"));
    rust_code.push_str("\n}\n");

    rust_code
}
