use std::{iter::Peekable, vec::IntoIter};

use super::lexer::{Token, TokenError};
use juax_lib::reg::{Reg, RegError};
use logos::Lexer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Invalid token {0}")]
    InvalidToken(String),

    #[error("Unexpected Token {0:?}")]
    UnexpectedToken(Token),

    #[error("Unexpected Eof")]
    UnexpectedEof,

    #[error("Token Error {0}")]
    TokenError(#[from] TokenError),

    #[error("Reg Error {0}")]
    RegError(#[from] RegError),
}

macro_rules! cmp {
    ($parser: expr, $pattern:pat) => {{
        if let Some(v) = $parser.peek() {
            $parser.next();
            match v {
                $pattern => v,
                _ => return Err(AstError::UnexpectedEof),
            }
        } else {
            return Err(AstError::UnexpectedEof);
        }
    }};
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Mov { to: Reg, from: Reg },
    Label { name: String, body: Vec<AstNode> },
}

pub struct Parser {
    pub tree: Vec<AstNode>,
    pub iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn peek(&mut self) -> Option<Token> {
        self.iter.peek().cloned()
    }
    pub fn bump(&mut self, node: AstNode) {
        self.tree.push(node);
        self.iter.next();
    }
    pub fn next(&mut self) {
        self.iter.next();
    }

    pub fn new(lex: &mut Lexer<'_, Token>) -> Result<Parser, AstError> {
        let mut iter = Vec::new();

        while let Some(token) = lex.next() {
            match token {
                Ok(v) => iter.push(v),
                Err(()) => {
                    return Err(AstError::InvalidToken(lex.slice().to_string()));
                }
            }
        }

        Ok(Parser {
            iter: iter.into_iter().peekable(),
            tree: Vec::new(),
        })
    }

    pub fn handle(&mut self) -> Result<(), AstError> {
        match self.peek() {
            Some(Token::Mov) => {
                self.next();
                let to = cmp!(self, Token::Register(_)).get_content()?;
                cmp!(self, Token::Comma);

                let from = cmp!(self, Token::Register(_)).get_content()?;

                self.bump(AstNode::Mov {
                    from: Reg::try_from(from)?,
                    to: Reg::try_from(to)?,
                });
            }
            Some(v) => {
                return Err(AstError::UnexpectedToken(v));
            }
            None => {}
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<AstNode>, AstError> {
        while let Some(_) = self.peek() {
            self.handle()?;
        }

        Ok(self.tree.clone())
    }
}
