use crate::parser::ast::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    MutableDecl,
    ConstantDecl,
    FunctionDef,
    Conditional,
    Backtick,
    InterpolationStart,
    TemplatePart(String),
    SiyahiKeyword,
    Else,
    Drop,
    TypeName(Type),
    Loop,
    BigInteger,
    LowInteger,
    Integer,
    String,
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Operator(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
    Comma,
    ListStart,
    ListEnd,
    Return,
    EOF,
    And,       // və
    Or,        // və ya
    DoubleAnd, // &&
    DoubleOr,  // ||
}
