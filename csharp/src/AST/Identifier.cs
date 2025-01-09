public class Identifier : IExpression
{
    public Token Token { get; }
    public string Value { get; }

    public Identifier(Token token, string value)
    {
        Token = token;
        Value = value;
    }

    public void ExpressionNode() { }

    public string TokenLiteral() => Token.Literal;
} 