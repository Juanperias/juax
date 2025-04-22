use clap::Parser;
use cli::Cli;
use logos::Logos;
use parser::lexer::Token;
use std::fs;

mod cli;
mod parser;
mod asm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.file)?;
    let mut lexer = Token::lexer(&content);
    while let Some(t) = lexer.next() {
        println!("{:?}", t);
    }
    

    Ok(())
}
