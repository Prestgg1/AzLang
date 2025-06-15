use crate::builtin::match_builtin;

use super::{Expr, Parser, Token};
use super::{
    call::parse_function_call, function::parse_function_def, list::parse_list,
    returnn::parse_return,
};

pub fn parse_expression(parser: &mut Parser, inside_function: bool) -> Result<Expr, String> {
    // Əvvəlcə baz expression parse edək
    parse_binary_op_expression(parser, inside_function, 1)
}

fn parse_primary_expression(parser: &mut Parser, inside_function: bool) -> Result<Expr, String> {
    let mut expr = match parser.peek() {
        Some(Token::StringLiteral(_)) => {
            if let Some(Token::StringLiteral(s)) = parser.next() {
                Expr::String(s.clone())
            } else {
                return Err("StringLiteral gözlənilirdi".to_string());
            }
        }
        Some(Token::Conditional) => return parse_if_expr(parser),
        Some(Token::Return) => return parse_return(parser, inside_function),
        Some(Token::FunctionDef) => return parse_function_def(parser),
        Some(Token::Number(_)) => {
            if let Some(Token::Number(val)) = parser.next() {
                Expr::FunctionCall {
                    name: "number".to_string(),
                    args: vec![Expr::String(val.to_string())],
                }
            } else {
                return Err("Ədəd gözlənilirdi".to_string());
            }
        }

        Some(Token::ListStart) => {
            parser.next(); // consume '['
            return parse_list(parser);
        }
        Some(Token::Loop) => {
            let loop_expr = parse_loop(parser)?;
            return Ok(loop_expr);
        }
        Some(Token::Identifier(_)) => {
            if let Some(Token::Identifier(id)) = parser.next().cloned() {
                let next_token = parser.peek();
                if let Some(Token::LParen) = next_token {
                    if match_builtin(&id).is_some() {
                        return parse_function_call(parser, &id);
                    }

                    parser.next(); // consume '('
                    let mut args = Vec::new();
                    loop {
                        match parser.peek() {
                            Some(Token::RParen) => {
                                parser.next(); // consume ')'
                                break;
                            }
                            Some(_) => {
                                let arg = parse_expression(parser, false)?;
                                args.push(arg);
                                if let Some(Token::Comma) = parser.peek() {
                                    parser.next();
                                }
                            }
                            None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
                        }
                    }

                    return Ok(Expr::FunctionCall {
                        name: id.clone(),
                        args,
                    });
                } else {
                    let mut expr = Expr::VariableRef(id.clone());

                    loop {
                        match parser.peek() {
                            Some(Token::ListStart) => {
                                parser.next(); // consume '['
                                let index_expr = parse_expression(parser, inside_function)?;
                                match parser.peek() {
                                    Some(Token::ListEnd) => {
                                        parser.next(); // consume ']'
                                        expr = Expr::Index {
                                            target: Box::new(expr),
                                            index: Box::new(index_expr),
                                        };
                                    }
                                    other => {
                                        return Err(format!(
                                            "Bağlanış ']' gözlənilirdi, tapıldı: {:?}",
                                            other
                                        ));
                                    }
                                }
                            }
                            _ => break,
                        }
                    }

                    expr
                }
            } else {
                return Err("Tanıtıcı gözlənilirdi".to_string());
            }
        }

        other => return Err(format!("Dəyər gözlənilirdi, tapıldı: {:?}", other)),
    };

    // İndi expr üzərində loop ilə .method() çağırışlarını parse edək
    loop {
        match parser.peek() {
            Some(Token::Operator(op)) if op == "." => {
                parser.next(); // consume Operator(".")
                // Method adı gəlməlidir
                println!("method_name: {:?}", parser.peek());
                let method_name = if let Some(Token::Identifier(name)) = parser.next() {
                    println!("method_name: {}", name);
                    name.clone()
                } else {
                    return Err("Method adı gözlənilirdi".to_string());
                };

                // Açıq mötərizə gəlməlidir
                if let Some(Token::LParen) = parser.next() {
                    let mut args = Vec::new();
                    loop {
                        match parser.peek() {
                            Some(Token::RParen) => {
                                parser.next(); // consume ')'
                                break;
                            }
                            Some(_) => {
                                let arg = parse_expression(parser, false)?;
                                args.push(arg);
                                if let Some(Token::Comma) = parser.peek() {
                                    parser.next();
                                }
                            }
                            None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
                        }
                    }

                    expr = Expr::MethodCall {
                        target: Box::new(expr),
                        method: method_name,
                        args,
                    };
                } else {
                    return Err("Funksiya çağırışı üçün '(' gözlənilirdi".to_string());
                }
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_loop(parser: &mut Parser) -> Result<Expr, String> {
    parser.next(); // consume `gəz`

    let var_name = if let Some(Token::Identifier(name)) = parser.next() {
        name.clone()
    } else {
        return Err("Dəyişən adı gözlənilirdi".to_string());
    };

    let iterable = parse_expression(parser, false)?;

    // Expect and consume the opening brace
    if parser.peek() != Some(&Token::LBrace) {
        return Err("Loop gövdəsi üçün '{' gözlənilirdi".to_string());
    }
    parser.next(); // consume {

    let mut body = Vec::new();
    while let Some(token) = parser.peek() {
        if token == &Token::RBrace {
            parser.next(); // consume }
            break;
        }

        let stmt = parse_expression(parser, true)?;
        body.push(stmt);

        // Consume semicolon after each statement except before RBrace
        if parser.peek() == Some(&Token::Semicolon) {
            parser.next();
        }
    }

    Ok(Expr::Loop {
        var_name,
        iterable: Box::new(iterable),
        body,
    })
}

pub fn parse_binary_op_expression(
    parser: &mut Parser,
    inside_function: bool,
    min_prec: u8,
) -> Result<Expr, String> {
    let mut left = parse_primary_expression(parser, inside_function)?;

    loop {
        let op_token = match parser.peek() {
            Some(Token::Operator(op)) => op.clone(),
            _ => break,
        };

        let prec = get_precedence(&op_token);
        if prec < min_prec {
            break;
        }

        parser.next(); // consume operator

        let mut right = parse_binary_op_expression(parser, inside_function, prec + 1)?;

        left = Expr::BinaryOp {
            left: Box::new(left),
            op: op_token,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn get_precedence(op: &str) -> u8 {
    match op {
        "||" => 1,
        "&&" => 2,
        "==" | "!=" => 3,
        "<" | ">" | "<=" | ">=" => 4,
        "+" | "-" => 5,
        "*" | "/" => 6,
        _ => 0,
    }
}
fn parse_if_expr(parser: &mut Parser) -> Result<Expr, String> {
    // Token::Conditional (əgər) gözlənilir
    match parser.next() {
        Some(Token::Conditional) => {}
        other => return Err(format!("'əgər' gözlənilirdi, tapıldı: {:?}", other)),
    }

    let condition = parse_expression(parser, false)?; // şərti ifadəni parse et

    // LBrace gözlənilir
    match parser.next() {
        Some(Token::LBrace) => {}
        other => return Err(format!("\n{{ gözlənilirdi, tapıldı: {:?}", other)),
    }

    let mut then_branch = Vec::new();

    while let Some(token) = parser.peek() {
        if token == &Token::RBrace {
            parser.next(); // consume }
            break;
        }

        let expr = parse_expression(parser, false)?;
        then_branch.push(expr);

        // opsional olaraq semicolon
        if parser.peek() == Some(&Token::Semicolon) {
            parser.next();
        }
    }

    let else_branch = if parser.peek() == Some(&Token::Else) {
        parser.next(); // consume `else`

        match parser.peek() {
            Some(Token::Conditional) => {
                // else if üçün parse_if_expr çağır
                let else_if_expr = parse_if_expr(parser)?;
                Some(vec![else_if_expr])
            }

            Some(Token::LBrace) => {
                parser.next(); // consume `{`

                let mut else_branch = Vec::new();
                while let Some(token) = parser.peek() {
                    if token == &Token::RBrace {
                        parser.next(); // consume }
                        break;
                    }

                    let expr = parse_expression(parser, false)?;
                    else_branch.push(expr);

                    if parser.peek() == Some(&Token::Semicolon) {
                        parser.next();
                    }
                }

                Some(else_branch)
            }

            other => {
                return Err(format!(
                    "'əgər' və ya '{{' gözlənilirdi (else üçün), tapıldı: {:?}",
                    other
                ));
            }
        }
    } else {
        None
    };

    Ok(Expr::If {
        condition: Box::new(condition),
        then_branch,
        else_branch,
    })
}
