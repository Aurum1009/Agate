namespace Scanner

open Utils
open System.Text
open ScanUtils

type public Scanner(source: string, fileName: string) =
    let Source = source
    let FileName = fileName
    let mutable Start = 0
    let mutable Current = 0
    let mutable Lines: int Range = Range(1, 1)
    let mutable Columns: int Range = Range(0, 0)
    let Length = source.Length
    let Keywords = Map.ofList [("true", TokenType.True); ("false", TokenType.False)]

    let IsAtEnd (): bool = Current >= Length
    let PeekTo distance =
        if Current + distance < Length then
            let char = Source.[Current + distance]
            Some char
        else
            None
    let Peek() = PeekTo(0)
    let PeekEq c = match Peek() with
                    | Some ch -> ch = c
                    | None -> false
    let Advance() =
        if Current < Length then
            let char = source.[Current]
            Current <- Current + 1

            if char = '\n' then
                Columns <- Range(Columns.Start, 0)
                Lines <- Range(Lines.Start, Lines.End + 1)
            else
                Columns <- Range(Columns.Start, Columns.End + 1)

            Some char
        else
            None
    let MakeToken kind: Token =
        Token(Source.[Start..Current], kind, Lines, Columns)

    let String(): ScanResult =
        while not (PeekEq('"')) do
            Advance() |> ignore

        if IsAtEnd() then
            Error (UnterminatedString(Lines.End, Columns.End))
        else
            // consume the closing quote
            Advance() |> ignore
            let string = if Current - Start <= 2 then "" else Source.[(Start + 1)..(Current - 1)]
            let token = Token(string, TokenType.StringLiteral, Lines, Columns)
            Success (token)
    
        /// Notes about this function for project devs: This uses an EarlyReturn exception to avoid certain F# semantics of passing values through blocks.
        /// Because F# does not allow you to return a value directly with a return statement, we use a `raise` and an `EarlyReturn` to simulate
        /// what an actual return statement would do in a language like Rust. 
        ///
        /// This is the only function to use these semantics because this is the only function that was unable to rearrange its structure to 
        /// fit F#'s semantics.
    let Char(): ScanResult =
        try
            let char = match Advance() with 
                            | Some ch -> ch
                            | None -> 
                                UnexpectedEndOfSource(Lines.End, Columns.End) |> Error |> EarlyRetun |> raise
            if char = '\\' then // Parse an escape sequence
                let next = match Advance() with 
                            | Some esc -> esc
                            | None ->
                                UnexpectedEndOfSource(Lines.End, Columns.End) |> Error |> EarlyRetun |> raise // There is no escaped character, like in " '\"
                let escape = match next with 
                                | '"' -> '\"'
                                | '\'' -> '\''
                                | '0' -> '\u0000' // null escape sequence
                                | 'n' -> '\n'
                                | 't' -> '\t'
                                | 'r' -> '\r'
                                | c -> 
                                    InvalidEscape(c, Lines.End, Columns.End) |> Error |> EarlyRetun |> raise // User inputted an invalid escape character
                Token(escape.ToString(), TokenType.CharLiteral, Lines, Columns) |> Success
            else
                Token(char.ToString(), TokenType.CharLiteral, Lines, Columns) |> Success
        with
            EarlyRetun ret -> ret // return an early return

    let ident(): Token =
        while match Peek() with
                    | Some ch -> isAlpha(ch)
                    | None -> false
                do
                Advance() |> ignore
        let ident = Source[Start..Current]
        let kind = match Keywords.TryFind(ident) with 
                        | Some kind -> kind
                        | None -> TokenType.Identifier
        MakeToken(kind)

    let ParseNumberToken kind: Token =
        let literal = Source[Start..Current]
        let parsed = literal.Replace("_", "")
        Token(parsed, kind, Lines, Columns)
    let NormalNumber(): Token =
        while match Peek() with
                | Some ch -> isNumber ch || ch = '_'
                | None -> false
            do
                Advance() |> ignore
        
        
        if PeekEq('.') && match PeekTo(1) with 
                                | Some ch -> isNumber ch 
                                | None -> false 
                then
                    Advance() |> ignore
                    while match Peek() with
                            | Some ch -> isNumber ch || ch = '_'
                            | None -> false
                        do
                            Advance() |> ignore
                    
                    if PeekEq('e') || PeekEq('E') then
                        Advance() |> ignore
                        if PeekEq('+') || PeekEq('-') then Advance() |> ignore
                        while match Peek() with
                                | Some ch -> isNumber(ch) || ch = '_'
                                | None -> false
                            do
                                Advance() |> ignore
                        ParseNumberToken TokenType.ScientificNumLiteral
                    else
                        ParseNumberToken TokenType.FloatLiteral
        else
            ParseNumberToken TokenType.IntegerLiteral
    let Number(): Token =
        if PeekEq('0') then
            Advance() |> ignore

            match Peek() with
                | Some 'b' | Some 'B' ->
                    Advance() |> ignore
                    while match Peek() with 
                            | Some ch -> isBinaryDigit ch || ch = '_'
                            | None -> false
                        do
                            Advance() |> ignore
                    ParseNumberToken TokenType.BinaryLiteral
                | Some 'o' | Some 'O' ->
                    Advance() |> ignore
                    while match Peek() with 
                            | Some ch -> isOctalDigit ch || ch = '_'
                            | None -> false
                        do
                            Advance() |> ignore
                    ParseNumberToken TokenType.OctalLiteral
                | Some 'x' | Some 'X' ->
                    Advance() |> ignore
                    while match Peek() with
                            | Some ch -> isHexadecimalDigit ch || ch = '_'
                            | None -> false
                        do
                            Advance() |> ignore
                    ParseNumberToken TokenType.HexadecimalLiteral
                | _ ->
                    NormalNumber()
        else NormalNumber()
        
        
