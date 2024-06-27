use logos::Logos;
use std::{fmt, num::ParseIntError};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+", error = LexicalError)]
pub enum Token {
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    Integer(i64),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("add")]
    OperatorAdd,
    #[token("sub")]
    OperatorSub,
    #[token("mul")]
    OperatorMul,
    #[token("div")]
    OperatorDiv,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
