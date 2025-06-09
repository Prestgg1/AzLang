use super::{Expr, Parser, Token}; // Token və Expr-i super (parser/mod.rs) vasitəsilə import edirik

// Tipləri emal edir (məsələn, ədəd, siyahı<ədəd>)
pub fn parse_type(parser: &mut Parser) -> Result<String, String> {
    let base_token = parser.next();
    let base = match base_token {
        Some(Token::TypeName(base)) => base.clone(),
        Some(Token::Identifier(base)) => base.clone(),
        Some(Token::Integer) => return Ok("integer".to_string()),
        Some(Token::String) => return Ok("string".to_string()),
        other => return Err(format!("Tip gözlənilirdi, tapıldı: {:?}", other)),
    };

    if let Some(Token::Operator(op)) = parser.peek() {
        if op == "<" {
            parser.next(); // consume '<'
            let inner = parse_type(parser)?; // Bu modulun öz funksiyasını rekursiv olaraq çağırırıq
            match parser.next() {
                Some(Token::Operator(op)) if op == ">" => {
                    return Ok(format!("{}<{}>", base, inner));
                }
                other => {
                    return Err(format!("Tip üçün '>' gözlənilirdi, tapıldı: {:?}", other));
                }
            }
        }
    }

    Ok(base)
}
