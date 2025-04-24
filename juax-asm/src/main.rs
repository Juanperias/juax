use cli::Cli;
use clap::Parser as CliParser;
use logos::Logos;
use parser::{ast::Parser, lexer::Token};
use std::fs;

mod cli;
mod parser;
mod asm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.file)?;
    let mut lexer = Token::lexer(&content);
    let parsed = Parser::new(&mut lexer).unwrap().parse().unwrap();
    
    println!("{:?}", parsed);
    

    Ok(())
}
