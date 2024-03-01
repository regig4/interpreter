use crate::lexer::Token;

trait Node {
    fn TokenLiteral() -> String;
}

pub(crate) trait Statement {
    fn statementNode() -> impl Node;
}

trait Expression {
    fn expressionNode() -> impl Node;
}

pub(crate) struct Program {
    statements: Vec<dyn Statement>
}

impl Program {
    fn token_literal(self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].TokenLiteral();
        }
        ""
    }
}

struct LetStatement {
    token: Token,
    name: *Identifier,
    value: dyn Expression
}

impl LetStatement {
    fn statement_node() {

    }

    fn token_literal(self) -> String {
        self.token.Literal
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