use super::Statement;
use crate::error::Error;
use crate::types::Type;

/// Funksiya başlığını və bədənini analiz edib `Statement::FunctionDef` formasına çevirir
pub fn parse_function(
    trimmed: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
) -> Result<Statement, Error> {
    // Başlıqdan "funksiya " prefiksini silirik
    let header = trimmed.strip_prefix("funksiya ").unwrap();

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
        // Parametri `ad: tip` formatında ayırırıq
        let parts: Vec<&str> = param.split(':').map(|s| s.trim()).collect();

        // Əgər format səhvdirsə error qaytarırıq
        if parts.len() != 2 {
            return Err(Error::InvalidParameterFormat(param.to_string()));
        }

        let param_name = parts[0].to_string();
        // Tipi `Type` enumuna çeviririk, əgər məlum tip deyilsə error veririk
        let param_type =
            Type::from_str(parts[1]).ok_or(Error::UnknownType(parts[1].to_string()))?;
        params.push((param_name, param_type));
    }

    let mut body = Vec::new();

    // Funksiya bədənini oxuyuruq - boş sətirləri keçirik
    while let Some(&next_line) = lines.peek() {
        if next_line.trim().is_empty() {
            lines.next();
            continue;
        }

        // Funksiya bədəni dörd boşluq və ya tab ilə indent olunmalıdır, əks halda funksiya bədəni bitmiş sayılır
        if !next_line.starts_with("    ") && !next_line.starts_with("\t") {
            break;
        }

        // Sətiri oxuyuruq və indentləri silirik
        let body_line = lines.next().unwrap().trim();

        // Yalnız `çap()` əmrlərini bədəndə qəbul edirik
        if body_line.starts_with("çap(") && body_line.ends_with(")") {
            let content = &body_line[4..body_line.len() - 1];
            body.push(Statement::Print(content.to_string()));
        } else {
            // Naməlum və ya dəstəklənməyən ifadə tapılarsa error
            return Err(Error::UnknownBodyStatement(body_line.to_string()));
        }
    }

    Ok(Statement::FunctionDef { name, params, body })
}
