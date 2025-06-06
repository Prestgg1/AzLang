pub mod block;
pub mod conditional;
pub mod function;
pub mod loops;
mod variable;
use crate::error::Error;
use crate::syntax::Syntax;
use crate::types::Type;
pub use block::parse_block;
pub use conditional::parse_conditional;
pub use function::parse_function;
pub use loops::parse_loop;
pub use variable::parse_variable;

#[derive(Debug)]
pub enum Statement {
    Print(String),
    Drop(String),
    FunctionDef {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        args: Vec<String>,
    },
    VariableDecl {
        mutable: bool,
        name: String,
        var_type: Type,
        value: String,
    },
    Conditional(Vec<conditional::Conditional>),
    Loop {
        iterator: String,
        iterable: String,
        body: Vec<Statement>,
    },
}

/// Kod mətnini parser vasitəsilə `Statement` massivinə çevirir
pub fn parse(code: &str, syntax: &Syntax) -> Result<Vec<Statement>, Error> {
    let mut statements = Vec::new();
    let mut lines = code.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with(&syntax.print) && trimmed.ends_with(")") {
            let content = &trimmed[syntax.print.len()..trimmed.len() - 1];
            statements.push(Statement::Print(content.to_string()));
        } else if trimmed.starts_with(&format!("{} ", syntax.conditional)) {
            statements.push(parse_conditional(trimmed, &mut lines, syntax)?);
        } else if trimmed.starts_with(&format!("{}(", syntax.drop)) && trimmed.ends_with(")") {
            let content = &trimmed[syntax.drop.len() + 1..trimmed.len() - 1];
            statements.push(Statement::Drop(content.to_string()));
        } else if trimmed.starts_with(&format!("{} ", syntax.function_def)) {
            statements.push(parse_function(trimmed, &mut lines, syntax)?);
        } else if trimmed.starts_with(&format!("{} ", syntax._loop)) {
            statements.push(parse_loop(trimmed, &mut lines, syntax)?);
        } else if trimmed.starts_with(&format!("{} ", syntax.mutable_decl))
            || trimmed.starts_with(&format!("{} ", syntax.constant_decl))
        {
            statements.push(parse_variable(trimmed, syntax)?);
        } else if trimmed.contains('(') && trimmed.ends_with(')') {
            let open_paren = trimmed.find('(').unwrap();
            let close_paren = trimmed.rfind(')').unwrap();
            let name = trimmed[..open_paren].trim().to_string();
            let args_str = &trimmed[open_paren + 1..close_paren];
            let args = args_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            statements.push(Statement::FunctionCall { name, args });
        } else if trimmed.is_empty() {
            continue;
        } else {
            return Err(Error::UnknownTopLevelStatement(trimmed.to_string()));
        }
    }

    Ok(statements)
}
