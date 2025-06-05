#[derive(Debug)]
pub enum Conditional {
    If {
        condition: String,
        body: Vec<super::Statement>,
    },
    ElseIf {
        condition: String,
        body: Vec<super::Statement>,
    },
    Else {
        body: Vec<super::Statement>,
    },
}

pub fn parse_conditional(
    first_line: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
    syntax: &crate::syntax::Syntax,
) -> Result<super::Statement, crate::error::Error> {
    use Conditional::*;

    let mut conditionals = Vec::new();

    let mut current_line = first_line.to_string();

    loop {
        let trimmed = current_line.trim();

        if trimmed.starts_with(&format!("{} ", syntax.conditional)) && trimmed.ends_with(":") {
            let condition = trimmed
                .trim_start_matches(&format!("{} ", syntax.conditional))
                .trim_end_matches(":")
                .trim();
            let body = super::parse_block(lines, syntax)?;
            conditionals.push(If {
                condition: condition.to_string(),
                body,
            });
        } else if trimmed.starts_with("yoxsa əgər ") && trimmed.ends_with(":") {
            let condition = trimmed
                .trim_start_matches("yoxsa əgər")
                .trim_end_matches(":")
                .trim();
            let body = super::parse_block(lines, syntax)?;
            conditionals.push(ElseIf {
                condition: condition.to_string(),
                body,
            });
        } else if trimmed.starts_with(&format!("{}:", syntax._else)) {
            let body = super::parse_block(lines, syntax)?;
            conditionals.push(Else { body });
            break;
        } else {
            break; // növbəti blok artıq şərt deyil
        }

        if let Some(next) = lines.peek() {
            current_line = next.to_string();
            if !(current_line.trim().starts_with("yoxsa")) {
                break;
            }
            lines.next();
        } else {
            break;
        }
    }

    Ok(super::Statement::Conditional(conditionals))
}
