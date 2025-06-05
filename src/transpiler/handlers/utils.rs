use super::context::Context;
use crate::types::Type;
use crate::warning::{Warning, warn};
use regex::Regex;
use std::collections::HashSet;

pub fn check_assignments_in_print(
    content: &str,
    mutable_vars: &HashSet<String>,
    used_as_assignment: &mut HashSet<String>,
) {
    for var in mutable_vars {
        let pattern = format!(r"\b{}\s*=", var);
        if Regex::new(&pattern).unwrap().is_match(content) {
            used_as_assignment.insert(var.clone());
        }
    }
}

pub fn check_usage_in_content(
    content: &str,
    params: &[(String, Type)],
    used_vars: &mut HashSet<String>,
) {
    for (param_name, _) in params {
        let pattern = format!(r"\b{}\b", param_name);
        if Regex::new(&pattern).unwrap().is_match(content) {
            used_vars.insert(param_name.clone());
        }
    }
}

pub fn warn_unused_params(params: &[(String, Type)], used_vars: &HashSet<String>) {
    for (name, _) in params {
        if !used_vars.contains(name) {
            warn(Warning::UnusedParam(name.clone()));
        }
    }
}

pub fn warn_unused_mutables(context: &Context) {
    for var in &context.mutable_vars {
        if !context.used_as_assignment.contains(var) {
            warn(Warning::MutableButNeverChanged(var.clone()));
        }
    }
}

pub fn handle_case_conversion(
    name: &str,
    args: &[String],
    context: &mut Context,
) -> Result<(), crate::error::Error> {
    if args.len() != 1 {
        return Err(crate::error::Error::InvalidArgumentCount(name.to_string()));
    }

    let func = match name {
        "böyüt" => "strings::upper::upper_case",
        "kiçilt" => "strings::lower::lower_case",
        _ => unreachable!(),
    };

    context
        .main_body
        .push(format!("    println!(\"{{}}\", {}({}));", func, args[0]));
    Ok(())
}

pub fn add_variable_declaration(
    code: &mut String,
    mutable: bool,
    name: &str,
    var_type: &Type,
    value: &str,
) {
    let rust_type = var_type.to_rust_type();
    if mutable {
        code.push_str(&format!(
            "    let mut {}: {} = {};\n",
            name, rust_type, value
        ));
    } else {
        if rust_type == "String" {
            code.push_str(&format!("    const {}: &str = {};\n", name, value));
        } else {
            code.push_str(&format!("    const {}: {} = {};\n", name, rust_type, value));
        }
    }
}

pub fn finalize_code(rust_code: &mut String, context: &Context) {
    rust_code.push_str(&context.functions.join(""));
    rust_code.push_str("fn main() {\n");
    rust_code.push_str(&context.main_body.join("\n"));
    rust_code.push_str("\n}\n");
}
