using Microsoft.VisualStudio.TestTools.UnitTesting;

[TestClass]
public class LexerTests
{
    [TestMethod]
    public void TestNextToken_BasicTokens()
    {
        var input = "=+(){},;";
        var lexer = new Lexer(input);

        var tests = new[]
        {
            (TokenType.Assign, "="),
            (TokenType.Plus, "+"),
            (TokenType.LParen, "("),
            (TokenType.RParen, ")"),
            (TokenType.LBrace, "{"),
            (TokenType.RBrace, "}"),
            (TokenType.Comma, ","),
            (TokenType.Semicolon, ";"),
            (TokenType.Eof, "\0")
        };

        foreach (var (expectedType, expectedLiteral) in tests)
        {
            var token = lexer.NextToken();
            Assert.AreEqual(expectedType, token.Type, $"Wrong token type. Expected {expectedType}, got {token.Type}");
            Assert.AreEqual(expectedLiteral, token.Literal, $"Wrong literal. Expected {expectedLiteral}, got {token.Literal}");
        }
    }

    [TestMethod]
    public void TestNextToken_ComplexMyProgram()
    {
        var input = @"
            let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };
            
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";

        var lexer = new Lexer(input);

        var tests = new[]
        {
            (TokenType.Let, "let"),
            (TokenType.Ident, "five"),
            (TokenType.Assign, "="),
            (TokenType.Int, "5"),
            (TokenType.Semicolon, ";"),
            (TokenType.Let, "let"),
            (TokenType.Ident, "ten"),
            (TokenType.Assign, "="),
            (TokenType.Int, "10"),
            (TokenType.Semicolon, ";"),
            (TokenType.Let, "let"),
            (TokenType.Ident, "add"),
            (TokenType.Assign, "="),
            (TokenType.Function, "fn"),
            (TokenType.LParen, "("),
            (TokenType.Ident, "x"),
            (TokenType.Comma, ","),
            (TokenType.Ident, "y"),
            (TokenType.RParen, ")"),
            (TokenType.LBrace, "{"),
            (TokenType.Ident, "x"),
            (TokenType.Plus, "+"),
            (TokenType.Ident, "y"),
            (TokenType.Semicolon, ";"),
            (TokenType.RBrace, "}"),
            (TokenType.Semicolon, ";"),
            // ... reszta tokenów
        };

        foreach (var (expectedType, expectedLiteral) in tests)
        {
            var token = lexer.NextToken();
            Assert.AreEqual(expectedType, token.Type, $"Wrong token type. Expected {expectedType}, got {token.Type}");
            Assert.AreEqual(expectedLiteral, token.Literal, $"Wrong literal. Expected {expectedLiteral}, got {token.Literal}");
        }
    }
} 