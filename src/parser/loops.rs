use super::{Statement, parse_block};
use crate::error::Error;
use crate::syntax::Syntax;

pub fn parse_loop(
    first_line: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
    syntax: &Syntax,
) -> Result<Statement, Error> {
    let trimmed = first_line.trim();

    if !trimmed.ends_with(":") {
        return Err(Error::InvalidSyntax(
            "Loop sətri ':' ilə bitməlidir".to_string(),
        ));
    }
    let header = trimmed
        .trim_end_matches(":")
        .trim_start_matches(&format!("{} ", syntax._loop));
    let mut parts = header.splitn(2, ' ');

    let iterator = parts
        .next()
        .ok_or_else(|| Error::InvalidSyntax("Iterator tapılmadı".to_string()))?
        .trim();

    let iterable = parts
        .next()
        .ok_or_else(|| Error::InvalidSyntax("Iterasiya ediləcək obyekt tapılmadı".to_string()))?
        .trim();

    let body = parse_block(lines, syntax)?;

    Ok(Statement::Loop {
        iterator: iterator.to_string(),
        iterable: iterable.to_string(),
        body,
    })
}
