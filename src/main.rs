// src/main.rs

mod ast;
mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let source = r#"
        int add(a, b) {
            return a + b;
        }

        int main() {
            int x = 10;
            int y = 20;
            int result = add(x, y);
            return result;
        }
    "#;

    println!("=== Source Code ===");
    println!("{}", source);

    // Lexical analysis
    println!("\n=== Lexing ===");
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("Lexing successful: {} tokens", tokens.len());
            tokens
        }
        Err(errors) => {
            for err in errors {
                eprintln!(
                    "Lexer error at {}:{}: {}",
                    err.line, err.column, err.message
                );
            }
            return;
        }
    };
    println!("{:?}", tokens);
}
