use asm::decode::decode_tree;
use clap::Parser as CliParser;
use cli::Cli;
use logos::Logos;
use parser::{ast::Parser, lexer::Token};
use std::fs;

mod asm;
mod cli;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.file)?;
    let mut lexer = Token::lexer(&content);
    let parsed = Parser::new(&mut lexer)?.parse()?;
    let jli = decode_tree(parsed)?;
    jli.save(cli.output)?;

    Ok(())
}
