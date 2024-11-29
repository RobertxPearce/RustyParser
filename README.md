# RustyParser

### Description
A simple lexer and parser written in the Rust programming language using the LL(1) grammar below. This grammar represents a small language with semicolon delimited statements and assignment, addition, and subtraction operations.

### Grammar
```               
<program> → <stmt_list>
<stmt_list> → <stmt> ; <stmt_list>
            | ϵ
<stmt> → <var> = <expr>
       | <expr>
<expr> → <term> <add_op>
<term> → <factor> <mul_op>
<factor> → <var>
         | <number>
         | ( <expr> )
<add_op> → + <expr>
         | - <expr>
         | ϵ
<mul_op> → * <term>
         | / <term>
         | ϵ
<var> → (A-Z | a-z)+
<number> → (0-9)+
```

## Files
```
rustyparser/
├─ src/
│  ├─ token.rs
│  ├─ lexer.rs
│  ├─ parser.rs
│  ├─ main.rs
│  ├─ tests/
│     ├─ test1.txt
│     ├─ test2.txt
│     ├─ test3.txt
├─ target/
├─ Cargo.toml
```
### token.rs
* `enum TokenTypes`: Enumeration for the types of tokens in language.
* `struct Token`: Struct to encapsulate token data such as type and line # for tokens made.
* `make_token(curr_line: usize, curr_type: TokenTypes) -> Token`: Function to make token given line # and type.

### lexer.rs
* `struct Lexer`: Struct to represent the Lexer with members for input, position, and line #.
* `new(input: String) -> Self`: Constructor to initialize Lexer given input string.
* `peek(&self) -> Option<char>`: Function to get char at the current position.
* `get_line(&self) -> usize`: Function to get the current line #.
* `skip_whitespace(&mut self)`: Function to skip whitespace (' ', '\t', '\n', '\r').
* `consume(&mut self)`: Function to "consume" the current lexeme (inc position).
* `consume_identifier(&mut self)`: Function to "consume" identifiers.
* `consume_number(&mut self)`: Function to "consume" numbers.
* `next_token(&mut self) -> Token`: Function to create and return the current token.

### parser.rs

### main.rs
* **LEXICAL ANALYSIS** - Initializes a lexer with an input string. It loops through generating tokens and pushing them into a vector terminating when the EOF token is reached.
* **SYNTAX ANALYSIS** -


* `print_header()`: Function to print a pretty header.
* `print_tokens(tokens: &Vec<Token>)`: Function to format and print tokens in a table.

### tests
* Directory for example programs to test the lexer and parser.

## Usage
Update file path: `let file_path = "src/tests/<test program>";`
```bash
cargo build
```
```bash
cargo run
```

## Resources
1. Sebesta, Robert W. Concepts of Programming Languages - 10th Edition. Pearson Addison Wesley, 2012.
    ```
    Figure 3.1 A Grammar for a Small Language

    <program> → begin <stmt_list> end 
    <stmt_list> → <stmt>
                  | <stmt> ; <stmt_list>
    <stmt> → <var> = <expression>
    <var> → A | B | C
    <expression> → <var> + <var>
                   | <var> – <var>
                   | <var>
    ```
    ```
    Figure 3.4 An Unambiguous Grammar for Expressions

    <assign> → <id> = <expr>
    <id> → A | B | C 
    <expr> → <expr> + <term>
             | <term>
    <term> → <term> * <factor>
             | <factor>
    <factor> → ( <expr> )
               | <id>
    ```