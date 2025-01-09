public class Lexer
{
    private readonly string _input;
    private int _position;
    private int _readPosition;
    private char _ch;
    
    private static readonly Dictionary<string, TokenType> _keywords = new()
    {
        {"fn", TokenType.Function},
        {"let", TokenType.Let},
        {"true", TokenType.True},
        {"false", TokenType.False},
        {"if", TokenType.If},
        {"else", TokenType.Else},
        {"return", TokenType.Return}
    };

    public Lexer(string input)
    {
        _input = input;
        _position = 0;
        _readPosition = 0;
        ReadChar();
    }

    private void ReadChar()
    {
        if (_readPosition >= _input.Length)
        {
            _ch = '\0';
        }
        else
        {
            _ch = _input[_readPosition];
        }
        _position = _readPosition;
        _readPosition++;
    }

    private char PeekChar()
    {
        if (_readPosition >= _input.Length)
            return '\0';
        return _input[_readPosition];
    }

    public Token NextToken()
    {
        SkipWhitespace();

        Token tok = _ch switch
        {
            '=' when PeekChar() == '=' => MakeToken(TokenType.Equals, "==", true),
            '=' => MakeToken(TokenType.Assign, "="),
            ';' => MakeToken(TokenType.Semicolon, ";"),
            '(' => MakeToken(TokenType.LParen, "("),
            ')' => MakeToken(TokenType.RParen, ")"),
            ',' => MakeToken(TokenType.Comma, ","),
            '+' => MakeToken(TokenType.Plus, "+"),
            '-' => MakeToken(TokenType.Minus, "-"),
            '!' when PeekChar() == '=' => MakeToken(TokenType.NotEquals, "!=", true),
            '!' => MakeToken(TokenType.Bang, "!"),
            '*' => MakeToken(TokenType.Asterisk, "*"),
            '/' => MakeToken(TokenType.Slash, "/"),
            '<' => MakeToken(TokenType.Lt, "<"),
            '>' => MakeToken(TokenType.Gt, ">"),
            '{' => MakeToken(TokenType.LBrace, "{"),
            '}' => MakeToken(TokenType.RBrace, "}"),
            '\0' => new Token(TokenType.Eof, ""),
            _ => HandleIdentifierOrNumber()
        };

        if (tok.Type != TokenType.Eof)
            ReadChar();
            
        return tok;
    }

    private Token HandleIdentifierOrNumber()
    {
        if (char.IsLetter(_ch))
        {
            string identifier = ReadIdentifier();
            TokenType type = LookupIdent(identifier);
            return new Token(type, identifier);
        }
        
        if (char.IsDigit(_ch))
        {
            string number = ReadNumber();
            return new Token(TokenType.Int, number);
        }

        return MakeToken(TokenType.Illegal, _ch.ToString());
    }

    private Token MakeToken(TokenType type, string literal, bool advanceExtra = false)
    {
        if (advanceExtra)
            ReadChar();
        return new Token(type, literal);
    }

    private void SkipWhitespace()
    {
        while (char.IsWhiteSpace(_ch))
        {
            ReadChar();
        }
    }

    private string ReadIdentifier()
    {
        int position = _position;
        while (char.IsLetter(_ch) || _ch == '_')
        {
            ReadChar();
        }
        return _input[position.._position];
    }

    private string ReadNumber()
    {
        int position = _position;
        while (char.IsDigit(_ch))
        {
            ReadChar();
        }
        return _input[position.._position];
    }

    private static TokenType LookupIdent(string ident)
    {
        return _keywords.TryGetValue(ident, out TokenType type) ? type : TokenType.Ident;
    }
} 