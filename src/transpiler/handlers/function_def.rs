use std::collections::HashSet;

use super::super::utils::add_variable_declaration;
use super::{Context, StatementHandler};
use crate::error::Error;
use crate::functions::{handle_drop, handle_print};
use crate::parser::Statement;
use crate::types::Type;

pub struct FunctionDefHandler<'a> {
    pub name: &'a str,
    pub params: &'a [(String, Type)],
    pub body: &'a [Statement],
}

impl StatementHandler for FunctionDefHandler<'_> {
    fn handle(&self, context: &mut Context) -> Result<(), Error> {
        let mut fn_code = String::new();
        let params_str = self
            .params
            .iter()
            .map(|(n, t)| format!("{}: {}", n, t.to_rust_type()))
            .collect::<Vec<_>>()
            .join(", ");

        fn_code.push_str(&format!("fn {}({}) {{\n", self.name, params_str));
        let mut used_vars = HashSet::new();

        for stmt in self.body {
            match stmt {
                Statement::Print(content) => {
                    fn_code.push_str(&format!("    {};\n", handle_print(content)));
                    // Burada check_usage_in_content funksiyası çağırılır
                    super::check_usage_in_content(content, self.params, &mut used_vars);
                }
                Statement::VariableDecl {
                    mutable,
                    name,
                    var_type,
                    value,
                } => {
                    add_variable_declaration(&mut fn_code, *mutable, name, var_type, value);
                }
                Statement::Drop(var) => {
                    fn_code.push_str(&format!("    {};\n", handle_drop(var)));
                }
                Statement::FunctionCall { name, args } => {
                    fn_code.push_str(&format!("    {}({});\n", name, args.join(", ")));
                }
                _ => return Err(Error::UnknownBodyStatement(format!("{:?}", stmt))),
            }
        }

        // warn_unused_params funksiyası utils modulunda olmalıdır
        super::super::utils::warn_unused_params(self.params, &used_vars);
        fn_code.push_str("}\n\n");
        context.functions.push(fn_code);
        Ok(())
    }
}
