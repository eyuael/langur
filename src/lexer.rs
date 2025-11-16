use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Tokens can be literals
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

     #[token("=")]
    Assign,

    // Regular expressions for more complex tokens
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),


    // Logo requires a token to handle whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}