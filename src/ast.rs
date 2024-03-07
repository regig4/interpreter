use std::any::Any;
use crate::lexer::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}

pub(crate) trait Statement {
    fn statement_node(&self) -> Box<dyn Node>;
    fn as_any(&self) -> &dyn Any;
}

trait Expression {
    fn expression_node(&self) -> Box<dyn Node>;
}

pub(crate) struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>
}

impl Program {
    fn token_literal(self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].statement_node().token_literal();
        }
        "".to_string()
    }
}

pub struct LetStatement {
    token: Token,
    name: *mut Identifier,
    value: dyn Expression
}

impl Statement for LetStatement {
    fn statement_node(&self) -> Box<dyn Node> {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn Any {
       self
    }
}

impl LetStatement {
    fn token_literal(&self) -> String {
        self.token.Literal.to_string()
    }
}

struct Identifier {
    token: Token,
    value: String
}

impl Identifier {
    fn expression_node() {

    }

    fn token_literal(self) -> String {
        self.token.Literal
    }
}