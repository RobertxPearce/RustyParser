/// Robert Pearce
/// Lexer Implementation

use crate::token;

///----------------------------------
///     LEXER

/// Struct for the lexer
pub struct Lexer {
    input: String,      // Variable for source code
    position: usize,    // Variable for current position
    line: usize,        // Variable for current line #
}

/// Implement methods for the Lexer
impl Lexer {
    /// @Brief Constructor
    /// @Param Source code as string
    /// @Return New instance of Lexer
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
        }
    }

    /// @Brief Function to return the next character in the input
    /// @Param Reference to self
    /// @Return A char or None with Option
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// @Brief Function to return the current line #
    /// @Param Reference to self
    /// @Return The current line #
    #[allow(dead_code)]
    pub fn get_line(&self) -> usize {
        self.line
    }

    /// @Brief Function to skip whitespace
    /// @Param Mutable reference to self to update position
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.position += 1;
            } else if c == '\n' || c == '\r' {
                self.position += 1;
                self.line += 1;
            } else {
                break;
            }
        }
    }

    /// @Brief Function to "consume" current lexeme
    /// @Param Mutable reference to self
    pub fn consume(&mut self) {
        self.position += 1;
    }

    /// @Brief Function to "consume" identifiers
    /// @Param Mutable reference to self
    fn consume_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                result.push(c);
                self.consume();
            } else {
                break;
            }
        }
        result
    }

    /// @Brief Function to "consume" numbers
    /// @Param Mutable reference to self
    fn consume_number(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                result.push(c);
                self.consume();
            } else {
                break;
            }
        }
        result
    }

    /// @Brief Function to return the current token
    /// @Param Reference to self mutable to update position
    /// @Return Current Token
    pub fn next_token(&mut self) -> token::Token {
        // Skip whitespace
        self.skip_whitespace();

        // Set char variable to next token
        let char = self.peek();

        // Switch statement to determine token type
        let curr_type = match char {
            //----- Identifier -----
            Some('a'..='z') | Some('A'..='Z') => {
                let identifier = self.consume_identifier();
                token::TokenTypes::TokIdentifier(identifier)
            },
            //----- Number -----
            Some('0'..='9') => {
                let number = self.consume_number();
                token::TokenTypes::TokNum(number)
            },
            //----- Operators -----
            Some(';') => {
                self.consume();
                token::TokenTypes::TokSemi
            },
            Some('=') => {
                self.consume();
                token::TokenTypes::TokAssign
            },
            Some('(') => {
                self.consume();
                token::TokenTypes::TokParenL
            },
            Some(')') => {
                self.consume();
                token::TokenTypes::TokParenR
            },
            Some('+') => {
                self.consume();
                token::TokenTypes::TokPlus
            },
            Some('-') => {
                self.consume();
                token::TokenTypes::TokMinus
            },
            Some('*') => {
                self.consume();
                token::TokenTypes::TokMul
            },
            Some('/') => {
                self.consume();
                token::TokenTypes::TokDiv
            },
            //----- Invalid -----
            Some(_) => {
                self.consume();
                token::TokenTypes::TokInvalid
            },
            None => {
                token::TokenTypes::TokEOF
            },
        };

        token::Token::make_token(self.line, curr_type)
    }

}  // End of Lexer methods
