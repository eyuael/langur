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

    // Regular expressions for more complex tokens
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),

    // Logo requires a token to handle whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}