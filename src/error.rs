use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Lex(String),
    Parse(String),
    Type(String),
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex(message) => write!(f, "字句解析エラー: {message}"),
            Self::Parse(message) => write!(f, "構文エラー: {message}"),
            Self::Type(message) => write!(f, "型エラー: {message}"),
            Self::Overflow => write!(f, "整数の範囲を超えました"),
        }
    }
}

impl std::error::Error for Error {}
