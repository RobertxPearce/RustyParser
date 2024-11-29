use crate::token::Token;

/// Robert Pearce
/// Main Implementation

mod token;
mod lexer;
mod parser;

fn main() {

    // Print pretty header
    print_header();

    // Variable for file path
    let file_path = "src/tests/test1.txt";

    // Read file contents to string variable
    let program = std::fs::read_to_string(file_path)
        .expect("Error: Invalid File")
        .replace("\r\n", "\n");

    //----------------------------------
    //     LEXICAL ANALYSIS

    // Create a new lexer instance with test string
    let mut lexer = lexer::Lexer::new(program);

    // Vector to hold the tokens
    let mut token_vec = Vec::new();

    // Loop through tokens until we reach TokEnd
    loop {
        // Variable for current token
        let token = lexer.next_token();

        // Push current token into vector
        token_vec.push(token.clone());

        // Exit loop if token is TokEnd
        if token.kind == token::TokenTypes::TokEOF {
            break;
        }
    }

    // Print tokens created by the lexer
    print_tokens(&token_vec);

    //----------------------------------
    //     SYNTAX ANALYSIS
}

/// @Brief Function to format and print tokens from lexer
/// @Param Vector of Tokens
pub fn print_header() {
    println!("{}", "=".repeat(40));
    println!("{:^40}", "RustyParser");
    println!("{:^40}", "Robert Pearce");
    println!("{}", "=".repeat(40));
    println!();
}


/// @Brief Function to format and print tokens from lexer
/// @Param Vector of Tokens
pub fn print_tokens(tokens: &Vec<Token>) {
    println!("{:<15} {:<10} {}", "Kind", "Line", "Details");
    println!("{:<15} {:<10} {}", "---------------", "----------", "-----------------");
    for token in tokens {
        match &token.kind {
            token::TokenTypes::TokIdentifier(name) => {
                println!("{:<15} {:<10} {}", "Identifier", token.line, name);
            }
            token::TokenTypes::TokNum(value) => {
                println!("{:<15} {:<10} {}", "Number", token.line, value);
            }
            token::TokenTypes::TokAssign => {
                println!("{:<15} {:<10} {}", "Assign", token.line, "");
            }
            token::TokenTypes::TokSemi => {
                println!("{:<15} {:<10} {}", "SemiColon", token.line, "");
            }
            token::TokenTypes::TokPlus => {
                println!("{:<15} {:<10} {}", "Plus", token.line, "");
            }
            token::TokenTypes::TokMinus => {
                println!("{:<15} {:<10} {}", "Minus", token.line, "");
            }
            token::TokenTypes::TokMul => {
                println!("{:<15} {:<10} {}", "Multiply", token.line, "");
            }
            token::TokenTypes::TokDiv => {
                println!("{:<15} {:<10} {}", "Divide", token.line, "");
            }
            token::TokenTypes::TokParenL => {
                println!("{:<15} {:<10} {}", "LeftParen", token.line, "");
            }
            token::TokenTypes::TokParenR => {
                println!("{:<15} {:<10} {}", "RightParen", token.line, "");
            }
            token::TokenTypes::TokInvalid => {
                println!("{:<15} {:<10} {}", "Invalid", token.line, "");
            }
            token::TokenTypes::TokEOF => {
                println!("{:<15} {:<10} {}", "EOF", token.line, "");
            }
        }
    }
}