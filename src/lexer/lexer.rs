use crate::Syntax;
use crate::parser::ast::Type;
use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;
use super::utils::skip_whitespace;

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub syntax: &'a Syntax,
    token_buffer: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, syntax: &'a Syntax) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            syntax,
            token_buffer: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.token_buffer.pop() {
            return Some(token);
        }

        skip_whitespace(&mut self.chars);

        let ch = *self.chars.peek()?;
        match ch {
            '(' => self.consume_char_and_return(Token::LParen),
            ')' => self.consume_char_and_return(Token::RParen),
            '{' => self.consume_char_and_return(Token::LBrace),
            ';' => self.consume_char_and_return(Token::Semicolon),
            ':' => self.consume_char_and_return(Token::Colon),
            ',' => self.consume_char_and_return(Token::Comma),
            '[' => self.consume_char_and_return(Token::ListStart),
            ']' => self.consume_char_and_return(Token::ListEnd),
            '`' => {
                self.chars.next();
                self.read_template_string()
            }
            '$' => self.read_template_string(),

            '}' => {
                self.chars.next();
                Some(Token::RBrace)
            }
            '"' => self.read_string(),
            '0'..='9' => self.read_number(),
            _ if ch.is_alphabetic() => self.read_word(),
            _ => self.read_operator(),
        }
    }

    fn consume_char_and_return(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    pub fn push_back_token(&mut self, token: Token) {
        self.token_buffer.push(token);
    }

    fn read_dollar(&mut self) -> Option<Token> {
        self.chars.next();
        self.read_template_string()
    }

    fn read_string(&mut self) -> Option<Token> {
        self.chars.next(); // Skip opening quote
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                self.chars.next(); // Skip closing quote
                return Some(Token::StringLiteral(string));
            }
            string.push(ch);
            self.chars.next();
        }

        None // Unterminated string
    }
    fn read_template_string(&mut self) -> Option<Token> {
        let mut content = String::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '`' => {
                    self.chars.next(); // Bitirici backtick
                    if !content.is_empty() {
                        return Some(Token::TemplatePart(content));
                    } else {
                        return Some(Token::Backtick);
                    }
                }
                '$' => {
                    let mut lookahead = self.chars.clone();
                    lookahead.next(); // $
                    if let Some('{') = lookahead.next() {
                        // Önce içerideki content varsa onu döndür
                        if !content.is_empty() {
                            let part = Token::TemplatePart(content);
                            content = String::new();
                            self.chars.next(); // $
                            self.chars.next(); // {
                            self.push_back_token(Token::InterpolationStart);
                            return Some(part);
                        }
                        self.chars.next(); // $
                        self.chars.next(); // {
                        return Some(Token::InterpolationStart);
                    } else {
                        // Normal $ karakteri
                        content.push('$');
                        self.chars.next();
                    }
                }
                '}' => {
                    self.chars.next(); // }
                    return Some(Token::RBrace);
                }
                _ => {
                    content.push(ch);
                    self.chars.next();
                }
            }
        }

        // Eğer döngü biterse ve hala content varsa
        if !content.is_empty() {
            Some(Token::TemplatePart(content))
        } else {
            None
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let mut num_str = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_digit(10) {
                num_str.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        num_str.parse().ok().map(Token::Number)
    }

    fn read_word(&mut self) -> Option<Token> {
        let mut word = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphabetic() || ch == '_' {
                word.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        // Keyword yoxlamaları

        if word == self.syntax.return_name {
            Some(Token::Return)
        } else if word == self.syntax.mutable_decl {
            Some(Token::MutableDecl)
        } else if word == self.syntax.constant_decl {
            Some(Token::ConstantDecl)
        } else if word == self.syntax.function_def {
            Some(Token::FunctionDef)
        } else if word == self.syntax.conditional {
            Some(Token::Conditional)
        } else if word == self.syntax._else {
            Some(Token::Else)
        } else if word == self.syntax._loop {
            Some(Token::Loop)
        } else if word == self.syntax.bool {
            return Some(Token::TypeName(Type::Bool));
        } else if word == self.syntax.listtype {
            return Some(Token::SiyahiKeyword);
        } else if word == self.syntax.biginteger {
            return Some(Token::TypeName(Type::BigInteger));
        } else if word == self.syntax.lowinteger {
            return Some(Token::TypeName(Type::LowInteger));
        } else if word == self.syntax.string {
            return Some(Token::TypeName(Type::Metn));
        } else if word == self.syntax.integer {
            return Some(Token::TypeName(Type::Integer));
        } else if self.syntax.is_type_str(&word) {
            return Some(Token::TypeName(Type::Istifadeci(word)));
        } else if word == self.syntax.string {
            return Some(Token::String);
        } else {
            Some(Token::Identifier(word))
        }
    }

    fn read_operator(&mut self) -> Option<Token> {
        let mut op = String::new();
        op.push(self.chars.next()?);

        // Çok karakterli operatörler için (örneğin ==, +=)
        if let Some(&next_ch) = self.chars.peek() {
            match (op.chars().next().unwrap(), next_ch) {
                ('=', '=')
                | ('!', '=')
                | ('<', '=')
                | ('>', '=')
                | ('+', '=')
                | ('-', '=')
                | ('*', '=')
                | ('/', '=')
                | ('&', '&')
                | ('|', '|') => {
                    op.push(next_ch);
                    self.chars.next();
                }
                _ => {}
            }
        }

        Some(Token::Operator(op))
    }
}
