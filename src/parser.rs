use crate::ast;
use crate::lexer::{Lexer, Token, TokenType};

pub struct Parser {
    l: Lexer,

    currToken: Token,
    peekToken: Token
}

impl Parser {
    pub(crate) fn new(l: Lexer) -> Parser {
        let p = Parser {
            l,
            currToken: Token { Type: TokenType::Illegal, Literal: "".to_string() },
            peekToken: Token { Type: TokenType::Illegal, Literal: "".to_string() },
        };

        p.nextToken();
        p.nextToken();

        p
    }

    fn nextToken(mut self) {
        self.currToken = self.peekToken;
        self.peekToken = self.l.next_token();
    }

    pub fn parse_program(self) -> Option<ast::Program> {
        None
    }
}