use std::collections::{HashMap};

#[derive(PartialEq, Eq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asteriks,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Equals,
    NotEquals,
}

pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}

fn lookup_ident(ident: &str) -> TokenType {
    let keywords = HashMap::from(
        [
            ("fn", TokenType::Function), 
            ("let", TokenType::Let),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("return", TokenType::Return),
        ]);

    if keywords.contains_key(ident) {
        let val = keywords.get(ident).unwrap();
        return (*val).clone();
    }
    return TokenType::Ident;
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        self.input[self.read_position]
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token;

        self.skip_whitespace();

        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    tok = new_token(TokenType::Equals, self.ch);
                    tok.Literal = String::from("==");
                } else {
                    tok = new_token(TokenType::Assign, self.ch);
                }
            },
            b';' => tok = new_token(TokenType::Semicolon, self.ch),
            b'(' => tok = new_token(TokenType::LParen, self.ch),
            b')' => tok = new_token(TokenType::RParen, self.ch),
            b',' => tok = new_token(TokenType::Comma, self.ch),
            b'+' => tok = new_token(TokenType::Plus, self.ch),
            b'-' => tok = new_token(TokenType::Minus, self.ch),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    tok = new_token(TokenType::NotEquals, self.ch);
                    tok.Literal = String::from("!=");
                } else {
                    tok = new_token(TokenType::Bang, self.ch);
                }
            }
            b'*' => tok = new_token(TokenType::Asteriks, self.ch),
            b'/' => tok = new_token(TokenType::Slash, self.ch),
            b'<' => tok = new_token(TokenType::Lt, self.ch),
            b'>' => tok = new_token(TokenType::Gt, self.ch),
            b'{' => tok = new_token(TokenType::LBrace, self.ch),
            b'}' => tok = new_token(TokenType::RBrace, self.ch),
            0 => tok = new_token(TokenType::Eof, 0),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    tok = new_token(TokenType::Illegal, self.ch);
                    tok.Literal = self.read_identifier();
                    tok.Type = lookup_ident(&tok.Literal);
                    return tok;
                } else if self.ch.is_ascii_digit() {
                    tok = new_token(TokenType::Illegal, self.ch);
                    tok.Literal = self.read_number();
                    tok.Type = TokenType::Int;
                    return tok;
                } else {
                    tok = new_token(TokenType::Illegal, self.ch);
                }
            }
        }

        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char()
        }
        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }
}

pub fn new_token(token_type: TokenType, ch: u8) -> Token {
    return Token {
        Type: token_type,
        Literal: String::from_utf8(vec![ch]).unwrap(),
    };
}
