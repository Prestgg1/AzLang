mod function;
mod variable;

use crate::error::Error;
use crate::types::Type;

pub use function::parse_function;
pub use variable::parse_variable;

#[derive(Debug)]
pub enum Statement {
    Print(String),
    Drop(String), // Yeni əlavə
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
}

/// Kod mətnini parser vasitəsilə `Statement` massivinə çevirir
pub fn parse(code: &str) -> Result<Vec<Statement>, Error> {
    let mut statements = Vec::new();
    let mut lines = code.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with("çap(") && trimmed.ends_with(")") {
            let content = &trimmed[4..trimmed.len() - 1];
            statements.push(Statement::Print(content.to_string()));
        } else if trimmed.starts_with("sil(") && trimmed.ends_with(")") {
            let content = &trimmed[4..trimmed.len() - 1];
            statements.push(Statement::Drop(content.to_string()));
        } else if trimmed.starts_with("funksiya ") {
            statements.push(parse_function(trimmed, &mut lines)?);
        } else if trimmed.starts_with("dəyişən ") || trimmed.starts_with("sabit ") {
            statements.push(parse_variable(trimmed)?);
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
