use super::{Expr, Parser, Token};

pub fn parse_expression(parser: &mut Parser, inside_function: bool) -> Result<Expr, String> {
    match parser.peek() {
        Some(Token::StringLiteral(_)) => {
            if let Some(Token::StringLiteral(s)) = parser.next() {
                Ok(Expr::String(s.clone()))
            } else {
                Err("StringLiteral gözlənilirdi".to_string())
            }
        }
        Some(Token::Return) => {
            if !inside_function {
                return Err("`return` yalnız funksiyanın içində istifadə oluna bilər".to_string());
            }

            parser.next(); // consume `return`
            let expr = parse_expression(parser, inside_function)?;
            Ok(Expr::Return(Box::new(expr)))
        }
        Some(Token::Print) => {
            parser.next(); // consume 'Print'

            match parser.next() {
                Some(Token::LParen) => {}
                other => return Err(format!("'(' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let arg = parse_expression(parser, inside_function)?;

            match parser.next() {
                Some(Token::RParen) => {}
                other => return Err(format!("')' gözlənilirdi, tapıldı: {:?}", other)),
            }

            Ok(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![arg],
            })
        }
        Some(Token::Input) => {
            parser.next(); // consume 'Input'

            match parser.next() {
                Some(Token::LParen) => {}
                other => return Err(format!("'(' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let arg = parse_expression(parser, false)?;

            match parser.next() {
                Some(Token::RParen) => {}
                other => return Err(format!(")' gözlənilirdi, tapıldı: {:?}", other)),
            }

            Ok(Expr::FunctionCall {
                name: "input".to_string(),
                args: vec![arg],
            })
        }
        Some(Token::FunctionDef) => {
            parser.next(); // consume 'function'

            let name = match parser.next() {
                Some(Token::Identifier(name)) => name.clone(),
                other => return Err(format!("Funksiya adı gözlənilirdi, tapıldı: {:?}", other)),
            };

            match parser.next() {
                Some(Token::LParen) => {}
                other => return Err(format!("'(' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let mut params: Vec<(String, String)> = Vec::new(); // (ad, tip)
            loop {
                let next_token = parser.peek();
                match next_token {
                    Some(Token::Identifier(_)) => {
                        let param_name = match parser.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            other => {
                                return Err(format!(
                                    "Parametr adı gözlənilirdi, tapıldı: {:?}",
                                    other
                                ));
                            }
                        };

                        // İndi : (Colon) gözləyirik
                        match parser.next() {
                            Some(Token::Colon) => {}
                            other => {
                                return Err(format!("':' gözlənilirdi, tapıldı: {:?}", other));
                            }
                        }

                        // Tip (məsələn: String, mətn, int və s.)
                        let param_type = match parser.next() {
                            Some(Token::TypeName(t)) => t.clone(),
                            other => {
                                return Err(format!(
                                    "Parametr tipi gözlənilirdi, tapıldı: {:?}",
                                    other
                                ));
                            }
                        };

                        params.push((param_name, param_type));

                        match parser.peek() {
                            Some(Token::Comma) => {
                                parser.next(); // vergülü götür
                            }
                            Some(Token::RParen) => {
                                // Parametr siyahısı bitdi
                            }
                            other => {
                                return Err(format!(
                                    "Parametrlər arasında ',' və ya ')' gözlənilirdi, tapıldı: {:?}",
                                    other
                                ));
                            }
                        }
                    }
                    Some(Token::RParen) => break, // parametr siyahısı bitdi
                    other => {
                        return Err(format!(
                            "Parametr adı və ya ')' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }
            }

            match parser.next() {
                Some(Token::RParen) => {}
                other => return Err(format!("')' gözlənilirdi, tapıldı: {:?}", other)),
            }

            match parser.next() {
                Some(Token::LBrace) => {}
                other => return Err(format!("'{{' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let mut body = Vec::new();
            loop {
                let next_token = parser.peek();
                match next_token {
                    Some(Token::RBrace) => {
                        parser.next(); // consume '}'
                        break;
                    }
                    Some(_) => {
                        let expr = parse_expression(parser, true)?;
                        body.push(expr);

                        let after_expr = parser.peek();
                        if let Some(Token::Semicolon) = after_expr {
                            parser.next();
                        }
                    }
                    None => return Err("Funksiya gövdəsi bağlanmadı".to_string()),
                }
            }

            Ok(Expr::FunctionDef { name, params, body })
        }

        Some(Token::Number(n)) => {
            if let Some(Token::Number(val)) = parser.next() {
                Ok(Expr::FunctionCall {
                    name: "number".to_string(),
                    args: vec![Expr::String(val.to_string())],
                })
            } else {
                Err("Ədəd gözlənilirdi".to_string())
            }
        }

        Some(Token::ListStart) => {
            parser.next(); // consume '['
            parse_list(parser)
        }

        Some(Token::Identifier(_)) => {
            if let Some(Token::Identifier(id)) = parser.next().cloned() {
                let next_token = parser.peek();
                if let Some(Token::LParen) = next_token {
                    parser.next(); // consume '('

                    let mut args = Vec::new();
                    loop {
                        let peek_token = parser.peek();
                        match peek_token {
                            Some(Token::RParen) => {
                                parser.next(); // consume ')'
                                break;
                            }
                            Some(_) => {
                                let arg = parse_expression(parser, false)?;
                                args.push(arg);

                                let after_arg = parser.peek();
                                if let Some(Token::Comma) = after_arg {
                                    parser.next(); // consume comma
                                }
                            }
                            None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
                        }
                    }

                    Ok(Expr::FunctionCall {
                        name: id.clone(),
                        args,
                    })
                } else {
                    Ok(Expr::VariableRef(id.clone()))
                }
            } else {
                Err("Tanıtıcı gözlənilirdi".to_string())
            }
        }

        other => Err(format!("Dəyər gözlənilirdi, tapıldı: {:?}", other)),
    }
}

// Siyahıları emal edir (məsələn, [1, 2, 3])
pub fn parse_list(parser: &mut Parser) -> Result<Expr, String> {
    let mut elements = Vec::new();

    loop {
        let next_token = parser.peek();
        match next_token {
            Some(Token::ListEnd) => {
                parser.next(); // consume ]
                break;
            }
            Some(_) => {
                let expr = parse_expression(parser, false)?;
                elements.push(expr);

                let after_expr = parser.peek();
                match after_expr {
                    Some(Token::Comma) => {
                        parser.next();
                    }
                    Some(Token::ListEnd) => continue,
                    other => {
                        return Err(format!(
                            "Siyahı üçün ',' və ya ']' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }
            }
            None => return Err("Siyahı bağlanmadı".to_string()),
        }
    }

    Ok(Expr::List(elements))
}
