public static class Repl
{
    private const string PROMPT = ">> ";

    public static void Start()
    {
        while (true)
        {
            Console.Write(PROMPT);
            var line = Console.ReadLine();
            
            if (string.IsNullOrEmpty(line))
                return;

            var lexer = new Lexer(line);
            var token = lexer.NextToken();

            while (token.Type != TokenType.Eof)
            {
                Console.WriteLine($"Type: {token.Type}, Literal: {token.Literal}");
                token = lexer.NextToken();
            }
        }
    }
} 