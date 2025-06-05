use super::Statement;
use crate::error::Error;
use crate::syntax::Syntax;

/// Kod blokunu (`{}` və ya indentlərlə olan hissə) oxuyur və `Statement` massivinə çevirir
pub fn parse_block(
    lines: &mut std::iter::Peekable<std::str::Lines>,
    syntax: &Syntax,
) -> Result<Vec<Statement>, Error> {
    let mut body = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            lines.next(); // boş sətiri keç
            continue;
        }

        // Əgər növbəti sətr conditional, function və ya başqa yuxarı səviyyəli blokdursa, dayandır
        if trimmed.starts_with(&syntax.conditional)
            || trimmed.starts_with(&syntax._else)
            || trimmed.starts_with(&syntax.function_def)
        {
            break;
        }

        let stmt = super::parse(line, syntax)?;
        body.extend(stmt);
        lines.next();
    }

    Ok(body)
}
