#[derive(Debug)]
pub enum Token<'a> {
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

const BUILTIN_FUNC_NAMES: [&str; 21] = [
    "atan2",
    "cos",
    "sin",
    "exp",
    "log",
    "sqrt",
    "int",
    "rand",
    "srand",
    "gsub",
    "index",
    "length",
    "match",
    "split",
    "sprintf",
    "sub",
    "substr",
    "tolower",
    "toupper",
    "close",
    "system",
];

/// (line, col)
#[derive(Debug)]
pub struct LexError(usize, usize);

pub struct Tokens<'a> {
    source: &'a str,
    idx: usize,
    line: usize,
    col: usize,
}

pub fn lex_tokens_from_string<'a>(source: &'a str) -> Tokens<'a> {
    Tokens {
        source,
        idx: 0,
        line: 1,
        col: 1,
    }
}

impl<'a> Tokens<'a> {
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

            Some(Token::String(&self.source[startidx..endidx]))
        } else {
            None
        }
    }

    fn lex_number_lit(&mut self) -> Option<Token<'a>> {
        let first_ch = self.peek_next_char()?;
        let starts_with_sign_then_digit = (first_ch == '+' || first_ch == '-') && matches!(self.char_at_idx(self.idx + 1)?, '0'..='9');
        let starts_with_digit = matches!(first_ch, '0'..='9');

        if starts_with_sign_then_digit || starts_with_digit {
            let chs = self.source.chars().skip(self.idx);
            let mut reject_dot = false;
            let mut reject_e = false;
            let mut reject_sign = false;

            let len = chs.take_while(|&ch| {
                if starts_with_sign_then_digit && !reject_sign {
                    reject_sign = true;
                    return true;
                }

                if matches!(ch, '0'..='9') {
                    return true;
                }

                if ch == '.' && !reject_dot {
                    reject_dot = true;
                    return true;
                }

                if ch == 'e' || ch == 'E' && !reject_e {
                    reject_e = true;
                    reject_dot = true;
                    return true;
                }

                false
            }).count();

            let lit = &self.source[self.idx..self.idx + len];
            self.idx += len;
            self.col += len;

            Some(Token::Number(lit))
        } else {
            None
        }
    }

    fn lex_ere(&mut self) -> Option<Token<'a>> {
        // TODO
        None
    }

    fn peek_ident(&mut self) -> Option<&'a str> {
        if matches!(self.peek_next_char()?, 'A'..='Z' | 'a'..='z' | '_') {
            let len = self.source
                .chars()
                .skip(self.idx)
                .take_while(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
                .count();

            Some(&self.source[self.idx..self.idx + len])
        } else {
            None
        }
    }

    /// Lex either a builtin or user-defined func name
    fn lex_func_name(&mut self) -> Option<Token<'a>> {
        let peeked_ident = self.peek_ident()?;
        if self.char_at_idx(self.idx + peeked_ident.len())? == '(' {
            self.idx += peeked_ident.len();
            self.col += peeked_ident.len();

            if BUILTIN_FUNC_NAMES.iter().any(|&x| x == peeked_ident) {
                Some(Token::BuiltinFuncName(peeked_ident))
            } else {
                Some(Token::FuncName(peeked_ident))
            }
        } else {
            None
        }
    }

    fn lex_keyword(&mut self) -> Option<Token<'a>> {
        let peeked_ident = self.peek_ident()?;
        let tkn = match peeked_ident {
            "BEGIN" => Token::Begin,
            "END" => Token::End,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "delete" => Token::Delete,
            "do" => Token::Do,
            "else" => Token::Else,
            "exit" => Token::Exit,
            "for" => Token::For,
            "function" => Token::Function,
            "if" => Token::If,
            "in" => Token::In,
            "next" => Token::Next,
            "print" => Token::Print,
            "printf" => Token::Printf,
            "return" => Token::Return,
            "while" => Token::While,
            "getline" => Token::Getline,
            _ => return None,
        };

        self.col += peeked_ident.len();
        self.idx += peeked_ident.len();

        Some(tkn)
    }

    fn lex_name(&mut self) -> Option<Token<'a>> {
        let peeked_ident = self.peek_ident()?;

        self.col += peeked_ident.len();
        self.idx += peeked_ident.len();

        Some(Token::Name(peeked_ident))
    }

    fn eat_space(&mut self) {
        let n = self.source
            .chars()
            .skip(self.idx)
            .take_while(|&ch| ch == ' ' || ch == '\t')
            .count();
        self.col += n;
        self.idx += n;
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let lexing_funcs = [
            Self::lex_string_lit,
            Self::lex_number_lit,
            Self::lex_ere,
            Self::lex_func_name,
            Self::lex_twochar_token,
            Self::lex_onechar_token,
            Self::lex_keyword,
            Self::lex_name,
        ];

        self.eat_space();

        if self.idx >= self.source.len() {
            None
        } else {
            for f in lexing_funcs {
                if let Some(token) = f(self) {
                    return Some(Ok(token));
                }
            }

            Some(Err(LexError(self.line, self.col)))
        }
    }
}
