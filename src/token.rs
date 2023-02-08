use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    Str(String),
    Number(f32),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: u32) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal: String = match &self.token_type {
            TokenType::Str(s) => s.to_string(),
            TokenType::Number(n) => n.to_string(),
            _ => "".to_owned(),
        };

        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}
