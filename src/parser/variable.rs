use super::Statement;
use crate::error::Error;
use crate::syntax::Syntax;
use crate::types::Type;

pub fn parse_variable(trimmed: &str, syntax: &Syntax) -> Result<Statement, Error> {
    let is_mutable = trimmed.starts_with(&syntax.mutable_decl);
    let prefix = if is_mutable {
        format!("{} ", &syntax.mutable_decl)
    } else {
        format!("{} ", &syntax.constant_decl)
    };
    let rest = trimmed.strip_prefix(&prefix).unwrap().trim();

    let parts: Vec<&str> = rest.split('=').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        return if is_mutable {
            Err(Error::InvalidVariableDeclaration(rest.to_string()))
        } else {
            Err(Error::InvalidConstantDeclaration(rest.to_string()))
        };
    }

    let left = parts[0];
    let raw_value = parts[1]; // ✨ raw_value saxlanılır, çünki çevirməliyik

    let (name, var_type) = if left.contains(':') {
        let left_parts: Vec<&str> = left.split(':').map(|s| s.trim()).collect();
        if left_parts.len() != 2 {
            return if is_mutable {
                Err(Error::InvalidVariableDeclaration(left.to_string()))
            } else {
                Err(Error::InvalidConstantDeclaration(left.to_string()))
            };
        }
        let name = left_parts[0].to_string();
        if !is_mutable && !name.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(Error::ConstantNameMustBeUppercase(name));
        }
        let var_type =
            Type::from_str(left_parts[1]).ok_or(Error::UnknownType(left_parts[1].to_string()))?;
        (name, var_type)
    } else {
        let name = left.to_string();
        if !is_mutable && !name.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(Error::ConstantNameMustBeUppercase(name));
        }
        (name, Type::Eded)
    };

    let value = var_type.to_rust_value(raw_value.trim(), !is_mutable);

    Ok(Statement::VariableDecl {
        mutable: is_mutable,
        name,
        var_type,
        value,
    })
}
