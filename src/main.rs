mod lexer;
mod token;

use lexer::Lexer;
use std::{env,fs};

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len()!=2{
        eprintln!("Usage: compiler <filename.cpp>");
        std::process::exit(1);
    }
    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .expect("failed to read the file");
    println!("Lexin {}...", filename);

    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();

    loop{
        let tok = lexer.next_token();
        tokens.push(tok.clone());
        if tok.kind == token::TokenKind::EOF{
            break;
        }
    }
    for t in &tokens{
        println!("{:?}", t);
    }
    let out = filename.replace(".cpp", ".tokens");
    let dump = tokens
        .iter()
        .map(|t| format!("{:?}\n",t))
        .collect::<String>();

        fs::write(&out, dump).expect("Failed to write the tokens");
        println!("Tokens written to {}", out);
}
