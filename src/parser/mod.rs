pub mod ast;

use crate::lexer::Token;
pub use ast::{Expr, Program};

// Digər modulları elan et
mod expressions;
mod statements;
mod types;
pub struct Parser {
    tokens: Vec<Token>,
    pub position: usize, // Testing üçün pub edək, sonra private ola bilər
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    // `peek` və `next` funksiyaları Parser-in özündə qalır,
    // çünki bunlar daxili vəziyyəti (position) idarə edir.
    pub fn peek(&self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        println!("peek() → position = {}, token = {:?}", self.position, tok);
        tok
    }

    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        println!("next() → position = {}, token = {:?}", self.position, tok);
        self.position += 1;
        tok
    }

    // Əsas parse funksiyası
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut expressions = Vec::new();
        while self.peek().is_some() {
            if let Some(expr) = self.parse_statement()? {
                // parse_statement çağırırıq
                expressions.push(expr);

                // Hər ifadədən sonra nöqtəli vergülü gözləyirik
                match self.peek() {
                    Some(Token::Semicolon) => {
                        self.next(); // Nöqtəli vergülü istehlak et
                    }
                    _ => return Err("İfadə nöqtəli vergüllə bitməlidir.".to_string()),
                }
            } else {
                // Heç bir ifadə təhlil olunmayıbsa, loop-dan çıx
                break;
            }
        }
        Ok(Program { expressions })
    }

    // Digər modullardakı funksiyaları buradan çağıracağıq
    // statements.rs
    fn parse_statement(&mut self) -> Result<Option<Expr>, String> {
        statements::parse_statement(self)
    }

    // expressions.rs
    fn parse_expression(&mut self) -> Result<Expr, String> {
        expressions::parse_expression(self, false)
    }
    /*     fn parse_list(&mut self) -> Result<Expr, String> {
        expressions::parse_list(self)
    } */

    // types.rs
    fn parse_type(&mut self) -> Result<String, String> {
        types::parse_type(self)
    }
}
