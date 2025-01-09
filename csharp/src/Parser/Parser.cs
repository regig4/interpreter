public class Parser
{
    private readonly Lexer _lexer;
    private Token _currentToken;
    private Token _peekToken;
    private readonly List<string> _errors = new();

    public IReadOnlyList<string> Errors => _errors;

    public Parser(Lexer lexer)
    {
        _lexer = lexer;
        
        // Odczytaj dwa tokeny, żeby ustawić currentToken i peekToken
        NextToken();
        NextToken();
    }

    private void NextToken()
    {
        _currentToken = _peekToken;
        _peekToken = _lexer.NextToken();
    }

    public MyProgram ParseMyProgram()
    {
        var MyProgram = new MyProgram();

        while (_currentToken.Type != TokenType.Eof)
        {
            var stmt = ParseStatement();
            if (stmt != null)
            {
                MyProgram.Statements.Add(stmt);
            }
            NextToken();
        }

        return MyProgram;
    }

    private IStatement ParseStatement()
    {
        return _currentToken.Type switch
        {
            TokenType.Let => ParseLetStatement(),
            _ => null
        };
    }

    private LetStatement ParseLetStatement()
    {
        var stmt = new LetStatement(_currentToken);

        if (!ExpectPeek(TokenType.Ident))
            return null;

        stmt.Name = new Identifier(_currentToken, _currentToken.Literal);

        if (!ExpectPeek(TokenType.Assign))
            return null;

        // TODO: Obecnie pomijamy wyrażenia do następnego średnika
        while (!CurrentTokenIs(TokenType.Semicolon))
            NextToken();

        return stmt;
    }

    private bool ExpectPeek(TokenType type)
    {
        if (PeekTokenIs(type))
        {
            NextToken();
            return true;
        }
        
        PeekError(type);
        return false;
    }

    private bool CurrentTokenIs(TokenType type) => _currentToken.Type == type;
    private bool PeekTokenIs(TokenType type) => _peekToken.Type == type;

    private void PeekError(TokenType type)
    {
        var msg = $"expected next token to be {type}, got {_peekToken.Type} instead";
        _errors.Add(msg);
    }
} 