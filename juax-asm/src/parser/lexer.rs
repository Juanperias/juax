use logos::Logos;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token without value {0:?}")]
    TokenWithoutValue(Token),
}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("mov")]
    Mov,

    #[regex("[a-zA-Z0-9]+", |lex| lex.slice().to_string())]
    Register(String),

    #[regex(r"-?\d+\s", |lex| {
        lex.slice().trim().to_string()
    })]
    Number(String),

    #[regex("[a-zA-Z0-9_]+:", |lex| { 
        let mut name = lex.slice().to_string();

        name.pop();

        name
    })]
    Label(String),

    #[token(",")]
    Comma,

    #[token("load")]
    Load,
}

impl Token {
    pub fn get_content(&self) -> Result<String, TokenError> {
        Ok(match self {
            Self::Register(reg) => reg.to_string(),
            Self::Label(label) => label.to_string(),
            Self::Number(n) => n.to_string(),
            _ => return Err(TokenError::TokenWithoutValue(self.clone())),
        })
    }
}
