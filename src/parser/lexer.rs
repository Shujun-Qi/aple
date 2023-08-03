use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token(".")]
    Period,

    #[token(",")]
    Comma,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(":-")]
    ImpliedBy,

    #[regex("[a-zA-Z_0-9]+")]
    Symbol,

    #[regex(r"\$[a-zA-Z_0-9]+", lex_variable)]
    Variable(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn lex_variable(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let n = slice[1..].to_string(); // skip '$'
    // println!("{}", n);
    Some(n)
}