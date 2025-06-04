use crate::error::Error;
use crate::functions::{handle_drop, handle_print};
use crate::parser::Statement;
use crate::warning::{Warning, warn};
use regex::Regex;
use std::collections::HashSet;

/// Verilmiş `Statement` massivini Rust koduna çevirir
pub fn transpile(statements: &[Statement]) -> Result<String, Error> {
    let mut rust_code = String::new(); // yekun Rust kodu burada toplanacaq
    let mut functions = Vec::new(); // funksiyalar ayrı toplanır
    let mut main_body = Vec::new(); // main funksiyasının gövdəsi
    rust_code.push_str("#![allow(warnings)]\n\n");
    // `let mut` ilə yaradılmış dəyişənlərin adlarını saxlayırıq
    let mut mutable_vars: HashSet<String> = HashSet::new();
    // dəyişdirilən (assignment edilən) dəyişənləri saxlayırıq
    let mut used_as_assignment: HashSet<String> = HashSet::new();

    for statement in statements {
        match statement {
            Statement::Print(content) => {
                // Çap əmri Rust-da println!() şəklində ifadə olunur
                main_body.push(format!("    {};", handle_print(content)));

                // Burada `print` içində dəyişənin dəyəri dəyişdirilibmi analiz edirik
                for var in &mutable_vars {
                    let pattern = format!(r"\b{}\s*=", var);
                    let re = Regex::new(&pattern).unwrap();

                    // Əgər çap mətnində dəyişənə yeni dəyər təyin olunursa qeyd edirik
                    if re.is_match(content) {
                        used_as_assignment.insert(var.clone());
                    }
                }
            }
            Statement::Drop(var) => {
                main_body.push(format!("    {}", handle_drop(var)));
            }

            Statement::FunctionDef { name, params, body } => {
                let mut fn_code = String::new();

                // Parametrləri Rust tip formatında string-ə çeviririk: `param: Tip`
                let params_str = params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t.to_rust_type()))
                    .collect::<Vec<_>>()
                    .join(", ");

                // Funksiya başlığı - məsələn `fn funksiya_adi(param1: i32) {`
                fn_code.push_str(&format!("fn {}({}) {{\n", name, params_str));

                // Parametrlərin istifadə olub-olmadığını yoxlamaq üçün Set
                let mut used_vars = HashSet::new();

                // Funksiya bədənində olan əmrlər
                for stmt in body {
                    if let Statement::Print(content) = stmt {
                        fn_code.push_str(&format!("    {};\n", handle_print(content)));

                        // Hər print əmri parametrlərdəki hansı adları istifadə edir?
                        for (param_name, _) in params {
                            let pattern = format!(r"\b{}\b", param_name);
                            let re = Regex::new(&pattern).unwrap();

                            if re.is_match(content) {
                                used_vars.insert(param_name.to_string());
                            }
                        }
                    }
                }

                // İstifadə olunmamış parametrlər barədə xəbərdarlıq veririk
                for (name, _) in params {
                    if !used_vars.contains(name) {
                        warn(Warning::UnusedParam(name.clone()));
                    }
                }

                fn_code.push_str("}\n\n");
                functions.push(fn_code);
            }

            Statement::FunctionCall { name, args } => {
                // Funksiya çağırışını Rust formatında əlavə edirik
                //Todo buraya baxarsan Problemlidir.
                match name.as_str() {
                    "böyüt" => {
                        if args.len() != 1 {
                            return Err(Error::InvalidArgumentCount(name.clone()));
                        }

                        main_body.push(format!(
                            "    println!(\"{{}}\", strings::upper::upper_case({}));",
                            args[0]
                        ));
                    }
                    "kiçilt" => {
                        if args.len() != 1 {
                            return Err(Error::InvalidArgumentCount(name.clone()));
                        }
                        // Assuming args[0] is a valid Rust expression string for the generated code
                        // and strings::lower::lower_case is available in the generated scope.
                        main_body.push(format!(
                            "    println!(\"{{}}\", strings::lower::lower_case({}));",
                            args[0]
                        ));
                    }
                    _ => {
                        main_body.push(format!("    {}({});", name, args.join(", ")));
                    }
                }
            }

            Statement::VariableDecl {
                mutable,
                name,
                var_type,
                value,
            } => {
                let rust_type = var_type.to_rust_type();
                if *mutable {
                    // Mutable dəyişən `let mut` şəklində Rust-da təyin olunur
                    mutable_vars.insert(name.clone());
                    main_body.push(format!("    let mut {}: {} = {};", name, rust_type, value));
                } else {
                    if rust_type == "String" {
                        // Sabit mətni &str kimi const olaraq çap et
                        main_body.push(format!("    const {}: &str = {};", name, value));
                    } else {
                        main_body.push(format!("    const {}: {} = {};", name, rust_type, value));
                    }
                }
            }
        }
    }

    // Mutable olub da heç vaxt dəyəri dəyişməyən dəyişənlər üçün xəbərdarlıq veririk
    for var in &mutable_vars {
        if !used_as_assignment.contains(var) {
            warn(Warning::MutableButNeverChanged(var.clone()));
        }
    }

    // Funksiyaları və main funksiyasını yekun Rust koduna əlavə edirik
    rust_code.push_str(&functions.join(""));
    rust_code.push_str("fn main() {\n");
    rust_code.push_str(&main_body.join("\n"));
    rust_code.push_str("\n}\n");

    Ok(rust_code)
}
