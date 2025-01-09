public class LetStatement : IStatement
{
    public Token Token { get; }
    public Identifier Name { get; set; }
    public IExpression Value { get; set; }

    public LetStatement(Token token)
    {
        Token = token;
    }

    public void StatementNode() { }

    public string TokenLiteral() => Token.Literal;
} 