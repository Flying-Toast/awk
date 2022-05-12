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

    fn char_at_idx(&self, idx: usize) -> Option<char> {
        self.source.chars().nth(idx)
    }

    fn peek_next_char(&self) -> Option<char> {
        self.char_at_idx(self.idx)
    }

    fn lex_onechar_token(&mut self) -> Option<Token<'a>> {
        let tkn = match self.peek_next_char()? {
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '\n' => Token::Newline,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '%' => Token::Percent,
            '^' => Token::Caret,
            '!' => Token::ExclamationPoint,
            '<' => Token::LeftAngleBracket,
            '>' => Token::RightAngleBracket,
            '|' => Token::Pipe,
            '?' => Token::QuestionMark,
            ':' => Token::Colon,
            '~' => Token::Tilde,
            '$' => Token::Dollar,
            '=' => Token::Equals,
            _ => return None,
        };

        self.idx += 1;
        if let Token::Newline = tkn {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(tkn)
    }

    fn lex_string_lit(&mut self) -> Option<Token<'a>> {
        if self.peek_next_char()? == '"' {
            let startidx = self.idx;
            let mut endidx = startidx + 1;
            let mut newcol = self.col + 2;

            let mut escaping = false;
            while escaping || self.char_at_idx(endidx)? != '"' {
                escaping = false;
                let curr = self.char_at_idx(endidx)?;
                if curr == '\n' {
                    // newlines not allowed in string literals
                    return None;
                }
                if curr == '\\' {
                    escaping = true;
                }
                endidx += 1;
                newcol += 1;
            }
            endidx += 1; // the closing quote

            self.idx = endidx;
            self.col = newcol;
            let lit = self.source.get(startidx..endidx).expect("lexed string is at valid source indices");

            Some(Token::String(lit))
        } else {
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            None
        } else if let Some(token) = self.lex_string_lit() {
            Some(Ok(token))
        } else if let Some(token) = self.lex_twochar_token() {
            Some(Ok(token))
        } else if let Some(token) = self.lex_onechar_token() {
            Some(Ok(token))
        } else {
            Some(Err(LexError(self.line, self.col)))
        }
    }
}

fn main() {
}
