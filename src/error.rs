use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// (line, col)
    Lex(usize, usize),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex(line, col) => write!(f, "lexer error at {line}:{col}"),
        }
    }
}
