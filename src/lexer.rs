use crate::error::{Result, Error};

#[derive(Debug, Copy, Clone)]
pub enum Token<'a> {
    Name(&'a str),
    Number(&'a str),
    String(&'a str),
    Ere(&'a str),
    FuncName(&'a str),
    BuiltinFuncName(BuiltinFunc),
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
    Slash,
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

#[derive(Debug, Copy, Clone)]
pub enum BuiltinFunc {
    Atan2,
    Cos,
    Sin,
    Exp,
    Log,
    Sqrt,
    Int,
    Rand,
    Srand,
    Gsub,
    Index,
    Length,
    Match,
    Split,
    Sprintf,
    Sub,
    Substr,
    Tolower,
    Toupper,
    Close,
    System,
}

pub struct Tokens<'a> {
    source: &'a str,
    last_non_newline: Option<Token<'a>>,
    idx: usize,
    line: usize,
    col: usize,
}

pub fn lex_tokens_from_string<'a>(source: &'a str) -> Tokens<'a> {
    Tokens {
        source,
        last_non_newline: None,
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
        Some(Token::String(self.lex_delimited_lit('"')?))
    }

    fn lex_delimited_lit(&mut self, delim: char) -> Option<&'a str> {
        if self.peek_next_char()? == delim {
            let startidx = self.idx;
            let mut endidx = startidx + 1;
            let mut newcol = self.col + 2;

            let mut escaping = false;
            while escaping || self.char_at_idx(endidx)? != delim {
                escaping = false;
                let curr = self.char_at_idx(endidx)?;
                if curr == '\n' {
                    // newlines not allowed
                    return None;
                }
                if curr == '\\' {
                    escaping = true;
                }
                endidx += 1;
                newcol += 1;
            }
            endidx += 1; // the closing delimiter

            self.idx = endidx;
            self.col = newcol;

            Some(&self.source[startidx..endidx])
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

    /// A slash can be division, the start of a DivAssign, or the start of an ERE.
    ///
    /// From https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html:
    ///
    /// There is a lexical ambiguity between the token ERE and the tokens '/' and DIV_ASSIGN.
    /// When an input sequence begins with a <slash> character in any syntactic context where
    /// the token '/' or DIV_ASSIGN could appear as the next token in a valid program, the
    /// longer of those two tokens that can be recognized shall be recognized. In any other
    /// syntactic context where the token ERE could appear as the next token in a valid program,
    /// the token ERE shall be recognized.
    fn lex_ambiguous_slash(&mut self) -> Option<Token<'a>> {
        if self.peek_next_char()? != '/' {
            return None;
        }

        if self.last_non_newline.is_some() && matches!(self.last_non_newline.unwrap(), Token::Name(_) | Token::Number(_) | Token::String(_) | Token::FuncName(_) | Token::BuiltinFuncName(_) | Token::Getline | Token::Incr | Token::Decr | Token::RightParen | Token::RightBracket | Token::Ere(_)) { // division is possible here
            if let Some("/=") = self.source.get(self.idx..self.idx + 2) {
                self.idx += 2;
                self.col += 2;

                Some(Token::DivAssign)
            } else {
                self.idx += 1;
                self.col += 1;

                Some(Token::Slash)
            }
        } else { // else assume ERE
            Some(Token::Ere(self.lex_delimited_lit('/')?))
        }
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

            let builtin = match peeked_ident {
                "atan2" => BuiltinFunc::Atan2,
                "cos" => BuiltinFunc::Cos,
                "sin" => BuiltinFunc::Sin,
                "exp" => BuiltinFunc::Exp,
                "log" => BuiltinFunc::Log,
                "sqrt" => BuiltinFunc::Sqrt,
                "int" => BuiltinFunc::Int,
                "rand" => BuiltinFunc::Rand,
                "srand" => BuiltinFunc::Srand,
                "gsub" => BuiltinFunc::Gsub,
                "index" => BuiltinFunc::Index,
                "length" => BuiltinFunc::Length,
                "match" => BuiltinFunc::Match,
                "split" => BuiltinFunc::Split,
                "sprintf" => BuiltinFunc::Sprintf,
                "sub" => BuiltinFunc::Sub,
                "substr" => BuiltinFunc::Substr,
                "tolower" => BuiltinFunc::Tolower,
                "toupper" => BuiltinFunc::Toupper,
                "close" => BuiltinFunc::Close,
                "system" => BuiltinFunc::System,
                _ => return Some(Token::FuncName(peeked_ident)),
            };

            Some(Token::BuiltinFuncName(builtin))
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
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let lexing_funcs = [
            Self::lex_string_lit,
            Self::lex_number_lit,
            Self::lex_ambiguous_slash,
            Self::lex_keyword,
            Self::lex_func_name,
            Self::lex_twochar_token,
            Self::lex_onechar_token,
            Self::lex_name,
        ];

        self.eat_space();

        if self.idx >= self.source.len() {
            None
        } else {
            for f in lexing_funcs {
                if let sometkn@Some(token) = f(self) {
                    if !matches!(token, Token::Newline) {
                        self.last_non_newline = sometkn;
                    }
                    return Some(Ok(token));
                }
            }

            Some(Err(Error::Lex(self.line, self.col)))
        }
    }
}
