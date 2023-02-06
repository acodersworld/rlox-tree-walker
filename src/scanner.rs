use crate::literal::Literal;
use crate::token::{Token, TokenType};
use phf::phf_map;
use std::str::CharIndices;
use std::vec::Vec;

#[derive(Clone)]
struct Cursor<'a> {
    chars: CharIndices<'a>,
    len: usize
}

impl<'a> Cursor<'a> {
    const EOF: char = '\0';

    fn new(chars: &str) -> Cursor {
        Cursor {
            chars: chars.char_indices(),
            len: chars.len()
        }
    }

    fn peek(&self) -> char {
        match self.chars.clone().peekable().peek() {
            None => Cursor::EOF,
            Some(&(_, c)) => c
        }
    }

    fn peek_next(&self) -> char {
        match self.chars.clone().next() {
            None => Cursor::EOF,
            Some((_, c)) => c
        }
    }

    fn index(&self) -> usize {
        match self.chars.clone().next() {
            None => self.len,
            Some((i, _)) => i
        }
    }

    fn advance(&mut self) -> char {
        let c = match self.chars.next() {
            None => Cursor::EOF,
            Some((_, c)) => c
        };
        println!("ch: {}", c);
        c
    }

    fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.peek()) {
            if self.advance() == Cursor::EOF {
                return
            }
        }
    }
}

struct Scanner<'a> {
    source: &'a str,
    start_cursor: Cursor<'a>,
    current_cursor: Cursor<'a>,
    line: u32,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
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
            source,
            start_cursor: Cursor::new(source),
            current_cursor: Cursor::new(source),
            line: 1,
            tokens: vec![],
        }
    }

    fn scan_tokens(&mut self) {
        while self.current_cursor.peek() != Cursor::EOF {
            self.start_cursor = self.current_cursor.clone();
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));
    }

    fn scan_token(&mut self) {
        let ch = self.current_cursor.advance();
        println!("SCAN '{}'", ch);

        match ch {
            Cursor::EOF => return,
            '\n' => {
                self.line += 1;
                return;
            }
            ch if ch.is_whitespace() => return,
            '/' => {
                if self.current_cursor.peek() == '/' {
                    self.current_cursor.eat_while(|c| c != '\n');
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            '"' => self.string(),
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

    fn get_token_string(&self) -> &str {
        &self.source[self.start_cursor.index()..self.current_cursor.index()]
    }

    fn number(&mut self) {
        self.current_cursor.eat_while(|c| c.is_ascii_digit());

        println!("FT {}", self.current_cursor.peek());
        if self.current_cursor.peek() == '.' {
            self.current_cursor.advance();
            self.current_cursor.eat_while(|c| c.is_ascii_digit());
        }

        let s = self.get_token_string();
        let value: f32 = s.parse().expect(&format!(
            "Expected token string to be a valid number. String: {}",
            s
        ));
        self.add_literal_token(TokenType::Number, Literal::Number(value));
    }

    fn identifier(&mut self) {
        self.current_cursor.eat_while(|c| c.is_alphanumeric());

        let s = self.get_token_string();
        if let Some(&token_type) = Scanner::KEYWORDS.get(&s) {
            self.add_token(token_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn string(&mut self) {
        println!("String");
        self.current_cursor.eat_while(|c| c != '"');

        let s = &self.source[self.start_cursor.index() + 1..self.current_cursor.index()];
        println!("SS {}", s);
        self.add_literal_token(TokenType::Str, Literal::Str(s.to_string()));
        self.current_cursor.advance();
    }

    fn match_char(
        &mut self,
        ch: char,
        matched_token: TokenType,
        unmatched_token: TokenType,
    ) -> TokenType {
        if self.current_cursor.peek() != ch {
            return unmatched_token;
        }

        self.current_cursor.advance();
        return matched_token;
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
