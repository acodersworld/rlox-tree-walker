use crate::literal::Literal;
use crate::token::{Token, TokenType};
use phf::phf_map;
use std::vec::Vec;

struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>,
}

impl Scanner {
    const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
        "comma" => TokenType::Comma,
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fun" => TokenType::Fun,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
    };

    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            ' ' | '\r' | '\t' => return,
            '\n' => {
                self.line += 1;
                return;
            }
            '/' => {
                if self.peek() == '/' {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => {
                self.string();
            }
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = self.match_char('=', TokenType::BangEqual, TokenType::Bang);
                self.add_token(token)
            }
            '=' => {
                let token = self.match_char('=', TokenType::EqualEqual, TokenType::Equal);
                self.add_token(token);
            }
            '>' => {
                let token = self.match_char('=', TokenType::GreaterEqual, TokenType::Greater);
                self.add_token(token);
            }
            '<' => {
                let token = self.match_char('=', TokenType::LessEqual, TokenType::Less);
                self.add_token(token);
            }
            _ => {
                if ch.is_ascii_digit() {
                    self.number()
                } else if ch.is_ascii_alphabetic() {
                    self.identifier()
                } else {
                    // error
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1];
    }

    fn add_token(&mut self, token_type: TokenType) {
        let s = self.get_token_string();
        self.tokens
            .push(Token::new(token_type, &s, None, self.line));
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Literal) {
        let s = self.get_token_string();
        self.tokens
            .push(Token::new(token_type, &s, Some(literal), self.line));
    }

    fn get_token_string(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let s = self.get_token_string();
        let value: f32 = s.parse().expect(&format!(
            "Expected token string to be a valid number. String: {}",
            s
        ));
        self.add_literal_token(TokenType::Number, Literal::Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let s = self.get_token_string();
        if let Some(&token_type) = Scanner::KEYWORDS.get(&s) {
            self.add_token(token_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' {
            self.advance();
        }

        let s = self.source[self.start + 1..self.current].iter().collect();
        self.add_literal_token(TokenType::Str, Literal::Str(s));
        self.advance();
    }

    fn match_char(
        &mut self,
        ch: char,
        matched_token: TokenType,
        unmatched_token: TokenType,
    ) -> TokenType {
        if self.is_at_end() {
            return unmatched_token;
        }
        if self.peek() != ch {
            return unmatched_token;
        }

        self.current += 1;
        return matched_token;
    }

    fn advance(&mut self) -> char {
        self.source[{
            let t = self.current;
            self.current += 1;
            t
        }]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let mut scanner = Scanner::new("");
        scanner.scan_tokens();

        assert!(scanner.tokens.len() == 1);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Eof);
        assert_eq!(scanner.tokens[0].line, 1);
    }

    #[test]
    fn comment() {
        let mut scanner = Scanner::new("// ==== ");
        scanner.scan_tokens();

        println!("{:?}", scanner.tokens);
        assert!(scanner.tokens.len() == 1);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn string() {
        let mut scanner = Scanner::new("\"a string\"");
        scanner.scan_tokens();

        println!("{:?}", scanner.tokens);
        assert!(scanner.tokens.len() == 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Str);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn spaces() {
        let mut scanner = Scanner::new(" \t\n\r");
        scanner.scan_tokens();

        assert!(scanner.tokens.len() == 1);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn newline() {
        let mut scanner = Scanner::new("\n");
        scanner.scan_tokens();

        assert!(scanner.tokens.len() == 1);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Eof);
        assert_eq!(scanner.tokens[0].line, 2);
    }

    #[test]
    fn integer() {
        let mut scanner = Scanner::new("1234");
        scanner.scan_tokens();

        assert!(scanner.tokens.len() == 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn float() {
        let mut scanner = Scanner::new("1234.5678");
        scanner.scan_tokens();

        assert!(scanner.tokens.len() == 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn scan() {
        let mut scanner = Scanner::new("<>===");
        scanner.scan_tokens();

        println!("{:?}", scanner.tokens);
        assert!(false);
    }
}
