using Microsoft.VisualStudio.TestTools.UnitTesting;

[TestClass]
public class ParserTests
{
    [TestMethod]
    public void TestLetStatements()
    {
        var input = @"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        var lexer = new Lexer(input);
        var parser = new Parser(lexer);

        var MyProgram = parser.ParseMyProgram();
        AssertNoParserErrors(parser);

        Assert.IsNotNull(MyProgram, "ParseMyProgram() returned null");
        Assert.AreEqual(3, MyProgram.Statements.Count, "MyProgram.Statements does not contain 3 statements");

        var expectedIdentifiers = new[] { "x", "y", "foobar" };

        for (var i = 0; i < expectedIdentifiers.Length; i++)
        {
            var stmt = MyProgram.Statements[i];
            TestLetStatement(stmt, expectedIdentifiers[i]);
        }
    }

    private void TestLetStatement(IStatement stmt, string expectedIdentifier)
    {
        Assert.AreEqual("let", stmt.TokenLiteral(), $"statement.TokenLiteral not 'let'. got={stmt.TokenLiteral()}");
        
        Assert.IsInstanceOfType(stmt, typeof(LetStatement), "stmt not LetStatement");
        var letStmt = (LetStatement)stmt;

        Assert.AreEqual(expectedIdentifier, letStmt.Name.Value, 
            $"letStmt.Name.Value not '{expectedIdentifier}'. got={letStmt.Name.Value}");
        
        Assert.AreEqual(expectedIdentifier, letStmt.Name.TokenLiteral(),
            $"letStmt.Name.TokenLiteral() not '{expectedIdentifier}'. got={letStmt.Name.TokenLiteral()}");
    }

    private void AssertNoParserErrors(Parser parser)
    {
        var errors = parser.Errors;
        if (errors.Count == 0) return;
        
        var messages = new System.Text.StringBuilder("parser has errors:\n");
        foreach (var error in errors)
        {
            messages.AppendLine($"parser error: {error}");
        }
        
        Assert.Fail(messages.ToString());
    }
} 