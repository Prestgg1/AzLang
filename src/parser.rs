// src/parser.rs

use crate::error::Error;

use crate::types::Type;

#[derive(Debug)]
pub enum Statement {
    Print(String),
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

pub fn parse(code: &str) -> Result<Vec<Statement>, Error> {
    let mut statements = Vec::new();
    let mut lines = code.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with("çap(") && trimmed.ends_with(")") {
            let content = &trimmed[4..trimmed.len() - 1];
            statements.push(Statement::Print(content.to_string()));
        } else if trimmed.starts_with("funksiya ") {
            let header = trimmed.strip_prefix("funksiya ").unwrap();
            let open_paren = header.find('(').ok_or(Error::InvalidFunctionHeader)?;
            let close_paren = header.find(')').ok_or(Error::InvalidFunctionHeader)?;

            let name = header[..open_paren].trim().to_string();
            let params_str = &header[open_paren + 1..close_paren];

            let mut params = Vec::new();
            for param in params_str.split(',') {
                let param = param.trim();
                if param.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = param.split(':').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(Error::InvalidParameterFormat(param.to_string()));
                }
                let param_name = parts[0].to_string();
                let param_type =
                    Type::from_str(parts[1]).ok_or(Error::UnknownType(parts[1].to_string()))?;
                params.push((param_name, param_type));
            }

            let mut body = Vec::new();
            while let Some(&next_line) = lines.peek() {
                if next_line.trim().is_empty() {
                    lines.next();
                    continue;
                }
                if !next_line.starts_with("    ") && !next_line.starts_with("\t") {
                    break;
                }
                let body_line = lines.next().unwrap().trim();
                if body_line.starts_with("çap(") && body_line.ends_with(")") {
                    let content = &body_line[4..body_line.len() - 1];
                    body.push(Statement::Print(content.to_string()));
                } else {
                    return Err(Error::UnknownBodyStatement(body_line.to_string()));
                }
            }

            statements.push(Statement::FunctionDef { name, params, body });
        } else if trimmed.starts_with("dəyişən ") {
            let rest = trimmed.strip_prefix("dəyişən ").unwrap().trim();
            let parts: Vec<&str> = rest.split('=').map(|s| s.trim()).collect();
            if parts.len() != 2 {
                return Err(Error::InvalidVariableDeclaration(rest.to_string()));
            }

            let left = parts[0]; // a və ya a:ədəd
            let value = parts[1]; // 3

            let (name, var_type) = if left.contains(':') {
                let left_parts: Vec<&str> = left.split(':').map(|s| s.trim()).collect();
                if left_parts.len() != 2 {
                    return Err(Error::InvalidVariableDeclaration(left.to_string()));
                }
                let name = left_parts[0].to_string();
                let var_type = Type::from_str(left_parts[1])
                    .ok_or(Error::UnknownType(left_parts[1].to_string()))?;
                (name, var_type)
            } else {
                (left.to_string(), Type::Eded)
            };

            statements.push(Statement::VariableDecl {
                mutable: true,
                name,
                var_type,
                value: value.to_string(),
            });
        } else if trimmed.starts_with("sabit ") {
            let rest = trimmed.strip_prefix("sabit ").unwrap().trim();
            let parts: Vec<&str> = rest.split('=').map(|s| s.trim()).collect();
            if parts.len() != 2 {
                return Err(Error::InvalidConstantDeclaration(rest.to_string()));
            }

            let left = parts[0]; // A və ya A:ədəd
            let value = parts[1];

            let (name, var_type) = if left.contains(':') {
                let left_parts: Vec<&str> = left.split(':').map(|s| s.trim()).collect();
                if left_parts.len() != 2 {
                    return Err(Error::InvalidConstantDeclaration(left.to_string()));
                }
                let name = left_parts[0].to_string();
                if !name.chars().all(|c| c.is_ascii_uppercase()) {
                    return Err(Error::ConstantNameMustBeUppercase(name));
                }
                let var_type = Type::from_str(left_parts[1])
                    .ok_or(Error::UnknownType(left_parts[1].to_string()))?;
                (name, var_type)
            } else {
                let name = left.to_string();
                if !name.chars().all(|c| c.is_ascii_uppercase()) {
                    return Err(Error::ConstantNameMustBeUppercase(name));
                }
                (name, Type::Eded)
            };

            statements.push(Statement::VariableDecl {
                mutable: false,
                name,
                var_type,
                value: value.to_string(),
            });
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
