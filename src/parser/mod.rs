pub mod ast;
pub mod validation;

use std::collections::HashSet;

use crate::{context::TranspileContext, lexer::Token, parser::ast::Type};
pub use ast::{Expr, Program};
pub use validation::validate_expr;

// Digər modulları elan et
pub mod call;
pub mod expressions;
pub mod function;
pub mod list;
pub mod returnn;

mod statements;
mod types;

pub struct Parser {
    tokens: Vec<Token>,
    pub position: usize, // Testing üçün pub edək, sonra private ola bilər
    pub declared_variables: HashSet<String>,
    pub used_variables: HashSet<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            declared_variables: HashSet::new(),
            used_variables: HashSet::new(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        /*         println!("peek() → position = {}, token = {:?}", self.position, tok);
         */
        tok
    }

    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        /*         println!("next() → position = {}, token = {:?}", self.position, tok);
         */
        self.position += 1;
        tok
    }

    /// Yalnız parse_program çağırılır
    pub fn parse(&mut self, ctx: &mut TranspileContext) -> Result<Program, String> {
        let expressions = self.parse_program(ctx)?;
        Ok(Program { expressions })
    }

    fn parse_statement(&mut self) -> Result<Option<Expr>, String> {
        statements::parse_statement(self)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        expressions::parse_expression(self, false)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        types::parse_type(self)
    }

    pub fn parse_program(&mut self, ctx: &mut TranspileContext) -> Result<Vec<Expr>, String> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            if let Some(stmt) = self.parse_statement()? {
                validate_expr(&stmt, ctx)?;
                statements.push(stmt);

                match self.peek() {
                    Some(Token::Semicolon) => {
                        self.next(); // İstehlak et
                    }
                    Some(tok) => {
                        return Err(format!(
                            "İfadə nöqtəli vergüllə bitməlidir. Tapıldı: {:?}",
                            tok
                        ));
                    }
                    None => return Err("Faylın sonunda `;` gözlənilirdi.".to_string()),
                }
            } else {
                break;
            }
        }

        // 🛑 İstifadə olunmayan dəyişənləri yoxla
        for var in &self.declared_variables {
            if !self.used_variables.contains(var) {
                return Err(format!("Dəyişən '{}' heç vaxt istifadə olunmayıb", var));
            }
        }

        Ok(statements)
    }
}
