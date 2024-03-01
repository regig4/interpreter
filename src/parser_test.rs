use ast::Statement;
use crate::ast;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

    let l = Lexer::new(String::from(input));
    let p = Parser::new(l);

    let program = p.parse_program();

    if program == None {
        panic!("parse_program() returned None");
    }
    if program.Statements.len != 3 {
        panic!("program.Statements does not contain 3 statements, got={}", len(program.Statements));
    }

    let tests: [TestStruct] =
    [
        TestStruct { expected_identifier: "x"},
        TestStruct { expected_identifier: "y"},
        TestStruct { expected_identifier: "foobar"},
    ];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = program.Statements[i];
        if !test_let_statement(stmt, tt.expected_identifier) {
            return;
        }
    }
}

fn test_let_statement(s: impl Statement, name: &str) -> bool {
    if s.TokenLiteral() != "let" {
        panic!("s.TokenLiteral not let, got {}", s);
        return false;
    }
}
struct TestStruct {
    expected_identifier: String
}