use ast::Statement;
use crate::ast;
use crate::ast::LetStatement;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

    let l = Lexer::new(String::from(input));
    let p = Parser::new(l);

    let program = p.parse_program();

    if program.is_none() {
        panic!("parse_program() returned None");
    }
    if program.unwrap().statements.len() != 3 {
        panic!("program.Statements does not contain 3 statements, got={}", program.unwrap().statements.len());
    }

    let tests: [TestStruct;3] =
    [
        TestStruct { expected_identifier: "x".to_string()},
        TestStruct { expected_identifier: "y".to_string()},
        TestStruct { expected_identifier: "foobar".to_string()},
    ];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.unwrap().statements[i];
        if !test_let_statement(stmt, &tt.expected_identifier) {
            return;
        }
    }
}

fn test_let_statement(s: &Box<dyn Statement>, name: &str) -> bool {
    if s.statement_node().token_literal() != "let" {
        println!("s.TokenLiteral not let, got {}", s.statement_node().token_literal());
        return false;
    }

    let let_stmt = s.as_any().downcast_ref::<LetStatement>();

    if !let_stmt.name.value.as_str().eq(name) {
        println!("let_stmt.name.value is not {}, got {}", name, let_stmt.name.value.as_str());
        return false;
    }

    if !let_stmt.token_literal.token_literal.as_str().eq(name) {
        println!("let_stmt.token_literal.value is not {}, got {}", name, let_stmt.token_literal.value.as_str());
        return false;
    }

    true
}
struct TestStruct {
    expected_identifier: String
}
