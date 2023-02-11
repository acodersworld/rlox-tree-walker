use crate::token::{Token, TokenType};
use phf::phf_map;
use std::str::CharIndices;
use std::vec::Vec;

pub fn scan(source: &str) -> Result<Vec<Token>, Vec<String>> {
    let mut chars = source.char_indices();
    let eof = (source.len(), '\0');
    let current = match chars.next() {
        None => (source.len(), '\0'),
        Some(x) => x,
    };

    let scanner = Scanner {
        source,
        chars,
        current,
        eof,
        line: 1,
        tokens: vec![],
        errors: vec![],
    };

    scanner.scan_tokens()
}

struct Scanner<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    current: (usize, char),
    eof: (usize, char),
    line: u32,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
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

    fn advance(&mut self) -> (usize, char) {
        let ret = self.current;
        self.current = match self.chars.next() {
            None => self.eof,
            Some(x) => x,
        };
        ret
    }

    fn advance_while(&mut self, predicate: impl Fn(char) -> bool) -> usize {
        while self.current != self.eof && predicate(self.current.1) {
            self.advance();
        }
        self.current.0
    }

    fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<String>> {
        while self.current != self.eof {
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, self.line));

        if self.errors.is_empty() {
            return Ok(self.tokens);
        }

        return Err(self.errors);
    }

    fn scan_token(&mut self) {
        let ch = self.advance();
        match ch.1 {
            '\n' => {
                self.line += 1;
                return;
            }
            ch if ch.is_whitespace() => return,
            '/' => {
                if self.current.1 == '/' {
                    self.advance_while(|c| c != '\n');
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
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
                if ch.1.is_ascii_digit() {
                    self.number(ch.0)
                } else if ch.1.is_ascii_alphabetic() || ch.1 == '_' {
                    self.identifier(ch.0)
                } else {
                    self.errors
                        .push(format!("Invalid character {} at line {}", ch.1, self.line));
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.line));
    }

    fn number(&mut self, start: usize) {
        let mut end = self.advance_while(|c| c.is_ascii_digit());

        let is_next_digit = || match self.chars.clone().next() {
            None => false,
            Some(x) => x.1.is_ascii_digit(),
        };

        if self.current.1 == '.' && is_next_digit() {
            self.advance();
            end = self.advance_while(|c| c.is_ascii_digit());
        }

        if self.current.1.is_ascii_alphabetic() {
            self.errors.push(format!("Unexpected character '{}' after number at line {}", self.current.1, self.line));
        }

        let s = &self.source[start..end];
        let value: f32 = s.parse().expect(&format!(
            "Expected token string to be a valid number. String: {}",
            s
        ));
        self.add_token(TokenType::Number(value));
    }

    fn identifier(&mut self, start: usize) {
        let end = self.advance_while(|c| c.is_alphanumeric() || c == '_');

        let s = &self.source[start..end];
        if let Some(token_type) = Scanner::KEYWORDS.get(&s) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::Identifier(s.to_string()));
        }
    }

    fn string(&mut self) {
        let start = self.current.0;
        let end = self.advance_while(|c| c != '"');

        let s = &self.source[start..end];
        self.add_token(TokenType::Str(s.to_string()));
        self.advance();
    }

    fn match_char(
        &mut self,
        ch: char,
        matched_token: TokenType,
        unmatched_token: TokenType,
    ) -> TokenType {
        if self.current.1 != ch {
            return unmatched_token;
        }

        self.advance();
        return matched_token;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let tokens = match scan("") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
        assert_eq!(tokens[0].line, 1);
    }

    #[test]
    fn comment() {
        let tokens = match scan("// ==== ") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn comment_new_line() {
        let tokens = match scan("// ==== \nNextLine") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("NextLine".to_owned())
        );
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn string() {
        let tokens = match scan("\"a string\"") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Str("a string".to_owned()));
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn spaces() {
        let tokens = match scan(" \t\n\r") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn newline() {
        let tokens = match scan("\n") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
        assert_eq!(tokens[0].line, 2);
    }

    #[test]
    fn integer() {
        let tokens = match scan("1234") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number(1234.0));
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn integer_method() {
        let tokens = match scan("1234.call") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number(1234.0));
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn float() {
        let tokens = match scan("1234.5678") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number(1234.5678));
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn float_1() {
        let tokens = match scan("1.2345") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number(1.2345));
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn error_if_alpha_after_number() {
        match scan("123ab") {
            Ok(_) => panic!("Should not scanner successfully"),
            Err(_) => {}
        };
    }

    #[test]
    fn non_alphanumeric() {
        let tokens = match scan("(){},.-+*/!!====<<=>>=") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        let expected_tokens = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::Equal,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Eof,
        ];

        for (i, t) in expected_tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *t);
        }

        assert_eq!(tokens.len(), expected_tokens.len());
    }

    #[test]
    fn error() {
        let errors = match scan("^&%") {
            Ok(_) => panic!("Expected scan error"),
            Err(e) => e,
        };

        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn indentifier_with_underscore() {
        let tokens = match scan("indentifier_with_underscore") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("indentifier_with_underscore".to_owned())
        );
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn indentifier_starts_with_underscore() {
        let tokens = match scan("_starts_with_underscore") {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("_starts_with_underscore".to_owned())
        );
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn keywords() {
        let mut source = "".to_owned();
        for (k, _) in &Scanner::KEYWORDS {
            source.push_str(k);
            source.push_str(" ");
        }

        let tokens = match scan(&source) {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(tokens.len(), Scanner::KEYWORDS.len() + 1);
        for (i, (_, v)) in Scanner::KEYWORDS.into_iter().enumerate() {
            assert_eq!(tokens[i].token_type, *v);
        }

        assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    }
}
