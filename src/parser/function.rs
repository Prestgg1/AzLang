use super::expressions::parse_expression;
use super::{Expr, Parser, Token};

pub fn parse_function_def(parser: &mut Parser) -> Result<Expr, String> {
    parser.next(); // consume FunctionDef

    let name = match parser.next() {
        Some(Token::Identifier(name)) => name.clone(),
        other => return Err(format!("Funksiya adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    if parser.next() != Some(&Token::LParen) {
        return Err("Funksiya parametr siyahısı '(' ilə başlamalıdır".to_string());
    }

    let mut params = Vec::new();
    loop {
        match parser.peek() {
            Some(Token::ConstantDecl) | Some(Token::MutableDecl) | Some(Token::Identifier(_)) => {
                let is_mutable = matches!(parser.peek(), Some(Token::MutableDecl));
                if is_mutable || matches!(parser.peek(), Some(Token::ConstantDecl)) {
                    parser.next();
                }

                let param_name = match parser.next() {
                    Some(Token::Identifier(name)) => name.clone(),
                    other => {
                        return Err(format!("Parametr adı gözlənilirdi, tapıldı: {:?}", other));
                    }
                };

                if parser.next() != Some(&Token::Colon) {
                    return Err("':' gözlənilirdi".to_string());
                }

                let param_type = match parser.next() {
                    Some(Token::TypeName(t)) => t.clone(),
                    other => {
                        return Err(format!("Parametr tipi gözlənilirdi, tapıldı: {:?}", other));
                    }
                };

                params.push((param_name, param_type, is_mutable));

                match parser.peek() {
                    Some(Token::Comma) => {
                        parser.next();
                    }
                    Some(Token::RParen) => {}
                    other => {
                        return Err(format!(
                            "Parametrlər arasında ',' və ya ')' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }
            }
            Some(Token::RParen) => break,
            other => {
                return Err(format!(
                    "Parametr adı və ya ')' gözlənilirdi, tapıldı: {:?}",
                    other
                ));
            }
        }
    }

    if parser.next() != Some(&Token::RParen) || parser.next() != Some(&Token::LBrace) {
        return Err("')' və ya '{' gözlənilirdi".to_string());
    }

    let mut body = Vec::new();
    loop {
        match parser.peek() {
            Some(Token::RBrace) => {
                parser.next();
                break;
            }
            Some(_) => {
                let expr = parse_expression(parser, true)?;
                body.push(expr);
                if matches!(parser.peek(), Some(Token::Semicolon)) {
                    parser.next();
                }
            }
            None => return Err("Funksiya gövdəsi bağlanmadı".to_string()),
        }
    }

    Ok(Expr::FunctionDef { name, params, body })
}
