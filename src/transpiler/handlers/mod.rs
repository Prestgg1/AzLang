mod conditional;
mod function_call;
mod function_def;
mod loops;
mod print;
mod variable_decl;
use super::Context;
use crate::error::Error;
use crate::parser::Statement;
use crate::transpiler::handlers::loops::LoopHandler;
use crate::types::Type;
use std::collections::HashSet;

use crate::functions::handle_drop;

pub trait StatementHandler {
    fn handle(&self, context: &mut Context) -> Result<(), Error>;
}

// Export all handler types
pub use conditional::ConditionalHandler;
pub use function_call::FunctionCallHandler;
pub use function_def::FunctionDefHandler;
pub use print::PrintHandler;
pub use variable_decl::VariableDeclHandler;

// Helper functions used by multiple handlers
pub(crate) fn check_usage_in_content(
    content: &str,
    params: &[(String, Type)],
    used_vars: &mut HashSet<String>,
) {
    use regex::Regex;

    for (param_name, _) in params {
        let pattern = format!(r"\b{}\b", param_name);
        if Regex::new(&pattern).unwrap().is_match(content) {
            used_vars.insert(param_name.clone());
        }
    }
}

/// Statement-i uyğun handler-ə yönləndirir
pub fn dispatch_statement(
    stmt: &Statement,
    context: &mut Context,
    _is_nested: bool, // Əgər nested blokdadırsa, gələcəkdə istifadə oluna bilər
) -> Result<(), Error> {
    match stmt {
        Statement::Print(content) => {
            PrintHandler(content).handle(context)?;
        }
        Statement::Drop(var) => {
            context.main_body.push(handle_drop(var));
        }
        Statement::FunctionDef { name, params, body } => {
            FunctionDefHandler { name, params, body }.handle(context)?;
        }
        Statement::FunctionCall { name, args } => {
            FunctionCallHandler { name, args }.handle(context)?;
        }
        Statement::VariableDecl {
            mutable,
            name,
            var_type,
            value,
        } => {
            VariableDeclHandler {
                mutable: *mutable,
                name,
                var_type,
                value,
            }
            .handle(context)?;
        }
        Statement::Conditional(conditionals) => {
            ConditionalHandler(conditionals).handle(context)?;
        }
        Statement::Loop {
            iterator,
            iterable,
            body,
        } => {
            LoopHandler {
                iterator,
                iterable,
                body,
            }
            .handle(context)?;
        }
    }

    Ok(())
}
