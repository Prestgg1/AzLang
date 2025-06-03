use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidFunctionHeader,
    InvalidParameterFormat(String),
    UnknownType(String),
    UnknownBodyStatement(String),
    UnknownTopLevelStatement(String),
    InvalidVariableDeclaration(String),
    ConstantNameMustBeUppercase(String),
    InvalidConstantDeclaration(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidFunctionHeader => write!(f, "\x1b[1;31mSəhv funksiya başlığı\x1b[0m"),
            Error::InvalidParameterFormat(param) => {
                write!(f, "\x1b[1;31mYanlış parametr formatı: {}\x1b[0m", param)
            }
            Error::UnknownType(ty) => write!(f, "\x1b[1;31mNaməlum tip: {}\x1b[0m", ty),
            Error::UnknownBodyStatement(stmt) => {
                write!(f, "\x1b[1;31mNaməlum bədən əmri: {}\x1b[0m", stmt)
            }
            Error::UnknownTopLevelStatement(stmt) => {
                write!(f, "\x1b[1;31mNaməlum əmrlə rastlaşdım: {}\x1b[0m", stmt)
            }
            Error::InvalidVariableDeclaration(stmt) => {
                write!(
                    f,
                    "\x1b[1;31mNaməlum dəyişən deklarasiyasi: {}\x1b[0m",
                    stmt
                )
            }
            Error::ConstantNameMustBeUppercase(stmt) => {
                write!(f, "\x1b[1;31mSabit adı BÜYÜK hərf olmalı: {}\x1b[0m", stmt)
            }
            Error::InvalidConstantDeclaration(stmt) => {
                write!(f, "\x1b[1;31mNaməlum sabit deklarasiyasi: {}\x1b[0m", stmt)
            }
        }
    }
}

impl std::error::Error for Error {}
