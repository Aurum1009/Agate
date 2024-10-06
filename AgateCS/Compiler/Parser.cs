using AgateCS.Scanner;
using AgateCS.Scanner.ScanUtils;
using AgateCS.Utils;

namespace AgateCS.Compiler
{
    public static class Parser
    {
        private static class Tokens
        {
            public static Token Current = Token.Eof();
            public static Token Previous = Token.Eof();

            private static IScanResult GetToken() => Scanner.Scanner.ScanToken();
            public static Token Advance()
            {
                for (; ; )
                {
                    IScanResult result = GetToken();
                    if (result is Token token)
                    {
                        Previous = Current;
                        Current = token;
                        return Current;
                    }
                    else
                    {
                        Console.Error.WriteLine("Scan Error: ", result.Message());
                    }
                }
            }
        }
        
        private static bool IsMain = false;
        private static string Source = "";
        private static ModuleAst Module = new("");
        private static bool HadError = false;

        public static void Init(string source, bool isMain = false)
        {
            Source = source;
            Scanner.Scanner.ReInit(source);
            IsMain = isMain;
        }
        private static Token Advance() => Tokens.Advance();

        public static void ParseItem()
        {
            throw new NotImplementedException();
        }
        public static bool Parse(ModuleAst module, bool isMain)
        {
            IsMain = isMain;
            HadError = false;
            Module = module;
            Advance();

            while (Tokens.Current.Type != TokenType.EOF)
            {
                ParseItem();
            }

            return HadError;
        }
    }
}
