use super::{Parser, Token};
use crate::parser::ast::Type; // `Type` enumunu import et

pub fn parse_type(parser: &mut Parser) -> Result<Type, String> {
    let base_token = parser.next();
    let base = match base_token {
        Some(Token::TypeName(t)) => t.clone(),
        Some(Token::Identifier(ident)) => Type::Istifadeci(ident.clone()),
        Some(Token::Integer) => Type::Integer,
        Some(Token::BigInteger) => Type::BigInteger,
        Some(Token::LowInteger) => Type::LowInteger,
        Some(Token::String) => Type::Metn,
        Some(Token::SiyahiKeyword) => {
            match parser.next() {
                Some(Token::Operator(op)) if op == "<" => (),
                other => return Err(format!("'<' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let inner_type = parse_type(parser)?;

            match parser.next() {
                Some(Token::Operator(op)) if op == ">" => (),
                other => return Err(format!("'>' gözlənilirdi, tapıldı: {:?}", other)),
            }

            Type::Siyahi(Box::new(inner_type))
        }
        other => return Err(format!("Tip gözlənilirdi, tapıldı: {:?}", other)),
    };

    Ok(base)
}
