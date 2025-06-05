mod context;
mod handlers;
mod utils;

use crate::error::Error;
use crate::parser::Statement;
use context::Context;
use handlers::*;

pub fn transpile(statements: &[Statement]) -> Result<String, Error> {
    let mut context = Context::new();
    let mut rust_code = String::new();
    rust_code.push_str("#![allow(warnings)]\n\n");

    for statement in statements {
        dispatch_statement(statement, &mut context, false)?;
    }

    utils::warn_unused_mutables(&context);
    utils::finalize_code(&mut rust_code, &context);
    Ok(rust_code)
}
