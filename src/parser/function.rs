use super::Statement;
use crate::error::Error;
use crate::parser::parse_variable;
use crate::syntax::Syntax;
use crate::types::Type;

/// Funksiya başlığını və bədənini analiz edib `Statement::FunctionDef` formasına çevirir
pub fn parse_function(
    trimmed: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
    syntax: &Syntax,
) -> Result<Statement, Error> {
    // Başlıqdan funksiya açar sözünü silirik
    let header = trimmed
        .strip_prefix(&syntax.function_def)
        .ok_or(Error::InvalidFunctionHeader)?;

    // Parametrlərin olduğu hissəni tapmaq üçün mötərizələrin indekslərini alırıq
    let open_paren = header.find('(').ok_or(Error::InvalidFunctionHeader)?;
    let close_paren = header.find(')').ok_or(Error::InvalidFunctionHeader)?;

    // Funksiya adını alırıq
    let name = header[..open_paren].trim().to_string();

    // Parametrlər stringini alırıq (məsələn: "x: i32, y: bool")
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

        // Funksiya bədəni indent olunmalıdır
        if !next_line.starts_with("    ") && !next_line.starts_with("\t") {
            break;
        }

        let body_line = lines.next().unwrap().trim();
        match body_line {
            line if line.starts_with(&format!("{}(", syntax.print)) && line.ends_with(")") => {
                let content = &line[syntax.print.len() + 1..line.len() - 1];
                body.push(Statement::Print(content.to_string()));
            }
            line if line.starts_with(&format!("{} ", syntax.mutable_decl))
                || line.starts_with(&format!("{} ", syntax.constant_decl)) =>
            {
                let stmt = parse_variable(line, syntax)?;
                body.push(stmt);
            }
            line => {
                return Err(Error::UnknownBodyStatement(line.to_string()));
            }
        }
    }

    Ok(Statement::FunctionDef { name, params, body })
}
