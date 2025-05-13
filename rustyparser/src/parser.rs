/// Robert Pearce
/// Parser Implementation

use crate::token;
use crate::lexer;
use crate::token::TokenTypes;

///----------------------------------
///     Parser

/// Struct for the Parser
pub struct Parser {
    lexer: lexer::Lexer,
    current_token: token::Token
}

/// Implement methods for the Parser
impl Parser {
    /// @Brief Constructor
    /// @Param Lexer
    /// @Return New instance of Parser
    pub fn new(mut lexer: crate::lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    /// @Brief Function to advance to next token
    /// @Param Reference to self
    /// @Return Next token
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// @Brief Function to match and consume token
    /// @Param Reference to self and token type
    /// @Return Error if incorrect token
    fn match_token(&mut self, expected::token:TokenTypes) -> bool {
        if self.current_token.kind == expected {
            self.advance();
            true
        } else {
            println!("Error: Found {:?} at line {}.",
                     self.current_token.kind, self.current_token.line);
            false
        }
    }

    fn peak_token(&mut self) {

    }

///------------------------
///    Parsing Functions
///------------------------

    /// @Brief
    /// @Param
    /// @Return
    fn parse_program(&mut self) -> bool {
        if (self.parse_stmt_list()) {
            true
        } else {
            println!("Error: Unexpected end of file");
            false
        }
    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_stmt_list(&mut self) -> bool {
        parse_stmt();
        self.match_token(TokenTypes::Semicolon);
        self.parse_stmt_list();
        true
    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_stmt(&mut self) -> bool {
        if () {
            parse_var();
            self.match_token(TokenTypes::TokAssign);
            self.advance();
            parse_expr();
            true
        } else {
            parse_expr();
            true
        }
        false
    }

    /// @Brief Parses an expression (term followed by optional add_op terms)
    /// @Param Reference to self
    /// @Return bool indicating if parsing was successful
    fn parse_expr(&mut self) -> bool {
        // First parse a term
        if !self.parse_term() {
            return false;
        }

        // Then parse zero or more (add_op term) pairs
        while self.current_token.kind == TokenTypes::Plus || 
              self.current_token.kind == TokenTypes::Minus {
            if !self.parse_add_op() {
                return false;
            }
            if !self.parse_term() {
                return false;
            }
        }

        true
    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_term(&mut self) -> bool {

    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_factor(&mut self) -> bool {

    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_add_op(&mut self) -> bool {

    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_mul_op(&mut self) -> bool {

    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_var(&mut self) -> bool {

    }

    /// @Brief
    /// @Param
    /// @Return
    fn parse_number(&mut self) -> bool {

    }

}  // End of Parser methods