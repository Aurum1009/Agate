namespace Scanner

open System
open Utils

module public ScanUtils =
    type public ScanError = 
        /// (character, line, column)
        | InvalidCharacter of char * int * int
        /// (line, column)
        | UnterminatedString of int * int 
        /// (line, column)
        | UnterminatedChar of int * int
        /// (line, column)
        | UnexpectedEndOfSource of int * int
        /// (suffix, line, column)
        | InvalidNumberSuffix of string * int * int
        /// (char, line, column)
        | InvalidEscape of char * int * int
        /// (type, line, column)
        | MalformedNumberLit of string * int * int

    type public ScanResult = 
        | Success of Token
        | Error of ScanError

    let public isLetter c = Char.IsLetter c
    let public isNumber c = Char.IsDigit c
    let public isAlpha c = isLetter(c) || c = '_'
    let public isBinaryDigit c = c = '0' || c = '1'
    let public isOctalDigit c = c >= '0' && c <= '7'
    let public isHexadecimalDigit c = (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')

    exception EarlyRetun of ScanResult