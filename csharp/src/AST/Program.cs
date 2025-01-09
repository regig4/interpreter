public class MyProgram : INode
{
    public List<IStatement> Statements { get; } = new();

    public string TokenLiteral()
    {
        if (Statements.Count > 0)
            return Statements[0].TokenLiteral();
        return string.Empty;
    }
} 