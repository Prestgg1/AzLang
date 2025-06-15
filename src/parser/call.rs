use crate::builtin::match_builtin;

use super::expressions::parse_expression;
use super::{Expr, Parser, Token};

pub fn parse_function_call(parser: &mut Parser, name: &str) -> Result<Expr, String> {
    if parser.next() != Some(&Token::LParen) {
        return Err("Function call üçün '(' gözlənilirdi".to_string());
    }

    let mut args = Vec::new();
    match parser.peek() {
        Some(Token::RParen) => {
            parser.next(); // consume ')'
        }
        Some(_) => {
            let arg = parse_expression(parser, false)?;
            args.push(arg);

            if parser.next() != Some(&Token::RParen) {
                return Err("Function call üçün ')' gözlənilirdi".to_string());
            }
        }
        None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
    }

    if let Some(builtin) = match_builtin(name) {
        Ok(Expr::BuiltInCall {
            func: builtin,
            args,
        })
    } else {
        Ok(Expr::FunctionCall {
            name: name.to_string(),
            args,
        })
    }
}
