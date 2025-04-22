use juax_lib::reg::Reg;
use logos::Lexer;
use thiserror::Error;
use super::lexer::Token; 

#[derive(Error, Debug)]
pub enum AstError {
    
}

#[derive(Debug)]
pub enum AstNode {
    Mov {
        dist: Reg,
        output: Reg,
    },
    Label {
        name: String,
        body: Vec<AstNode>
    }
}

pub fn process_from_tokens(lex: &mut Lexer<'_, Token>) -> Vec<AstNode> {
    let mut nodes = Vec::new();

    while let Some(token) = lex.next() {
        
    }

    nodes
}
