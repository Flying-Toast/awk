enum Token<'a> {
    Name(&'a str),
    Number(&'a str),
    String(&'a str),
    Ere(&'a str),
    FuncName(&'a str),
    Begin,
    End,
    Break,
    Continue,
    Delete,
    Do,
    Else,
    Exit,
    For,
    Function,
    If,
    In,
    Next,
    Print,
    Printf,
    Return,
    While,
    BuiltinFuncName(&'a str),
    Getline,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    Or,
    And,
    NoMatch,
    Eq,
    Le,
    Ge,
    Ne,
    Incr,
    Decr,
    Append,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    LeftParen,
    RightParen,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    Comma,
    Semicolon,
    Newline,
    Plus,
    Minus,
    Asterisk,
    Percent,
    Caret,
    ExclamationPoint,
    /// <
    LeftAngleBracket,
    /// >
    RightAngleBracket,
    /// |
    Pipe,
    QuestionMark,
    Colon,
    Tilde,
    Dollar,
    Equals,
}

/// (line, col)
struct LexError(usize, usize);

struct Lexer<'a> {
    source: &'a str,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    fn lex_string(source: &'a str) -> Self {
        Self {
            source,
            line: 1,
            col: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            None
        } else {
            todo!()
        }
    }
}

fn main() {
}
