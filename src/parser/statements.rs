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
    if parser.declared_variables.contains(&name) {
        return Err(format!("Dəyişən '{}' artıq əvvəl təyin olunub", name));
    }
    parser.declared_variables.insert(name.clone());

    parser.used_variables.insert(name.clone());
    let typ = match parser.peek() {
        Some(Token::Colon) => {
            parser.next(); // ':' consume
            Some(parser.parse_type()?)
        }
        _ => None,
    };

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

    // constant ilə input istifadə olunmamalıdır
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

    // input yalnız dəyişən mənimsədilməsində istifadə oluna bilər
    if let Expr::FunctionCall { name, .. } = &expr {
        if name == "input" {
            return Err("input yalnız dəyişən mənimsədilməsində istifadə oluna bilər.".to_string());
        }
    }

    // VariableRef varsa qeyd et
    record_variable_usage(&expr, &mut parser.used_variables);

    Ok(Some(expr))
}

fn record_variable_usage(expr: &Expr, used: &mut std::collections::HashSet<String>) {
    match expr {
        Expr::VariableRef(name) => {
            used.insert(name.clone());
        }
        Expr::FunctionCall { args, .. } | Expr::BuiltInCall { args, .. } | Expr::List(args) => {
            for arg in args {
                record_variable_usage(arg, used);
            }
        }
        Expr::MethodCall { target, args, .. } => {
            record_variable_usage(target, used);
            for arg in args {
                record_variable_usage(arg, used);
            }
        }
        Expr::Return(inner) | Expr::Index { target: inner, .. } => {
            record_variable_usage(inner, used);
        }
        Expr::MutableDecl { value, .. } | Expr::ConstantDecl { value, .. } => {
            record_variable_usage(value, used);
        }
        _ => {}
    }
}
