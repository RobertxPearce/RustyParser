/// Robert Pearce
/// Token Implementation

///----------------------------------
///     TOKEN


/// Enumeration for the tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TokenTypes {
    //--------------
    //  Operators
    TokSemi,        // ;
    TokAssign,      // =
    TokParenL,      // (
    TokParenR,      // )

    TokPlus,        // +
    TokMinus,       // -
    TokMul,         // *
    TokDiv,         // /

    //--------------
    //    Other
    TokIdentifier(String),  // Identifier
    TokNum(String),         // Number
    TokEOF,                 // End of File
    TokInvalid,             // Error
}

/// Struct for token type
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenTypes,   // Variable for token type
    pub line: usize,        // Variable for line of token
}

/// Implement methods for use with Token
impl Token {
    /// @Brief Function to create and return token
    /// @Param Reference to lexer
    /// @Param Token Type
    pub fn make_token(curr_line: usize, curr_type: TokenTypes) -> Token {
        Token {
            kind: curr_type,
            line: curr_line,
        }
    }

}  // End of Token methods