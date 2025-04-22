use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("mov")]
    Mov,

    #[regex("[a-zA-Z0-9]+", |lex| lex.slice().to_string())]
    Register(String),

    #[regex("[a-zA-Z0-9_]+:", |lex| { 
        let mut name = lex.slice().to_string();

        name.pop();

        name
    })]
    Label(String),

    #[token(",")]
    Comma
}
