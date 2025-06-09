use super::{Expr, Parser, Token}; // Token və Expr-i super (parser/mod.rs) vasitəsilə import edirik

// Dəyişən elanlarını emal edir (dəyişən a:ədəd = 10; kimi)
pub fn parse_variable_declaration(parser: &mut Parser, kind: &str) -> Result<Option<Expr>, String> {
    let name = match parser.next() {
        Some(Token::Identifier(name)) => name.clone(),
        other => {
            return Err(format!(
                "{} üçün dəyişən adı gözlənilirdi, tapıldı: {:?}",
                kind, other
            ));
        }
    };

    match parser.next() {
        Some(Token::Colon) => {}
        other => {
            return Err(format!(
                "{} üçün ':' gözlənilirdi, tapıldı: {:?}",
                kind, other
            ));
        }
    }
    let typ = parser.parse_type()?; // Parser metodunu çağırırıq

    match parser.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        other => {
            return Err(format!(
                "{} üçün '=' operatoru gözlənilirdi, tapıldı: {:?}",
                kind, other
            ));
        }
    }

    let value_expr = parser.parse_expression()?; // Parser metodunu çağırırıq

    if kind == "constant_decl" {
        if let Expr::FunctionCall { name, .. } = &value_expr {
            if name == "input" {
                return Err(
                    "input yalnız dəyişən (mutable) mənimsədilməsində istifadə oluna bilər."
                        .to_string(),
                );
            }
        }
    }
    match kind {
        "mutable_decl" => Ok(Some(Expr::MutableDecl {
            name,
            typ,
            value: Box::new(value_expr),
        })),
        "constant_decl" => Ok(Some(Expr::ConstantDecl {
            name,
            typ,
            value: Box::new(value_expr),
        })),
        _ => unreachable!(),
    }
}

// "çap(arg)" kimi funksiya çağırışlarını emal edir
pub fn parse_print_call(parser: &mut Parser) -> Result<Option<Expr>, String> {
    match parser.next() {
        Some(Token::LParen) => {
            let arg = parser.parse_expression()?; // Parser metodunu çağırırıq
            match parser.next() {
                Some(Token::RParen) => Ok(Some(Expr::FunctionCall {
                    name: "print".to_string(),
                    args: vec![arg],
                })),
                other => Err(format!(
                    "Bağlanış mötərizəsi gözlənilirdi, tapıldı: {:?}",
                    other
                )),
            }
        }
        other => Err(format!(
            "Açılış mötərizəsi gözlənilirdi, tapıldı: {:?}",
            other
        )),
    }
}

// Bu, proqramımızdakı "sətr"lər, yəni ifadələrdir
pub fn parse_statement(parser: &mut Parser) -> Result<Option<Expr>, String> {
    match parser.peek() {
        Some(Token::MutableDecl) | Some(Token::ConstantDecl) => {
            let kind = parser.next().unwrap();
            let kind_str = match kind {
                Token::MutableDecl => "mutable_decl",
                Token::ConstantDecl => "constant_decl",
                _ => unreachable!(),
            };
            parse_variable_declaration(parser, kind_str) // Bu modulun öz funksiyasını çağırırıq
        }
        Some(Token::Print) => {
            parser.next();
            parse_print_call(parser) // Bu modulun öz funksiyasını çağırırıq
        }
        // Gələcəkdə bura yeni ifadə tipləri əlavə edə bilərik:
        // Some(Token::If) => parse_if_statement(parser),
        // Some(Token::For) => parse_for_loop(parser),
        // Some(Token::While) => parse_while_loop(parser),
        // Some(Token::Return) => parse_return_statement(parser),
        _ => parse_expression_as_statement(parser), // Bu modulun öz funksiyasını çağırırıq
    }
}

// parse_expression-dan gələn dəyəri sadəcə ifadə kimi emal edir
pub fn parse_expression_as_statement(parser: &mut Parser) -> Result<Option<Expr>, String> {
    let expr = parser.parse_expression()?; // Parser metodunu çağırırıq

    // input("...") bir dəyişənə mənimsədilməlidir, təkbaşına ifadə ola bilməz
    if let Expr::FunctionCall { name, .. } = &expr {
        if name == "input" {
            return Err("input yalnız dəyişən mənimsədilməsində istifadə oluna bilər.".to_string());
        }
    }

    Ok(Some(expr))
}
