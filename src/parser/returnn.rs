use super::expressions::parse_expression;
use super::{Expr, Parser};

pub fn parse_return(parser: &mut Parser, inside_function: bool) -> Result<Expr, String> {
    if !inside_function {
        return Err("`return` yalnız funksiyanın içində istifadə oluna bilər".to_string());
    }

    parser.next();
    let expr = parse_expression(parser, inside_function)?;
    Ok(Expr::Return(Box::new(expr)))
}
