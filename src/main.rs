#[derive(Debug)]
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
#[derive(Debug)]
struct LexError(usize, usize);

struct Lexer<'a> {
    source: &'a str,
    idx: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn lex_string(source: &'a str) -> Self {
        Self {
            source,
            idx: 0,
            line: 1,
            col: 1,
        }
    }

    fn lex_twochar_token(&mut self) -> Option<Token<'a>> {
        let tkn = match self.source.get(self.idx..self.idx + 2)? {
            "+=" => Token::AddAssign,
            "-=" => Token::SubAssign,
            "*=" => Token::MulAssign,
            "/=" => Token::DivAssign,
            "%=" => Token::ModAssign,
            "^=" => Token::PowAssign,
            "||" => Token::Or,
            "&&" => Token::And,
            "!~" => Token::NoMatch,
            "==" => Token::Eq,
            "<=" => Token::Le,
            ">=" => Token::Ge,
            "!=" => Token::Ne,
            "++" => Token::Incr,
            "--" => Token::Decr,
            ">>" => Token::Append,
            _ => return None,
        };

        self.idx += 2;
        self.col += 2;

        Some(tkn)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            return None;
        } else if let Some(token) = self.lex_twochar_token() {
            return Some(Ok(token));
        } else {
            todo!()
        }
    }
}

fn main() {
}
