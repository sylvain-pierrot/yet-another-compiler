use logos::{Logos, Span};

type Error = (String, Span);

type Result<T> = std::result::Result<T, Error>;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("(")]
    BracketOpen,

    #[token(")")]
    BracketClose,

    #[token("add")]
    OperationAdd,

    #[token("sub")]
    OperationSub,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
}

pub fn tokenizer(input: String) -> Result<Vec<Token>> {
    let mut lexer = Token::lexer(input.as_str());

    let mut tokens = Vec::new();
    let _span = lexer.span();
    // let mut awaits_bracket = false;

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::BracketOpen) => tokens.push(Token::BracketOpen),
            Ok(Token::BracketClose) => tokens.push(Token::BracketClose),
            Ok(Token::OperationAdd) => tokens.push(Token::OperationAdd),
            Ok(Token::OperationSub) => tokens.push(Token::OperationSub),
            Ok(Token::Number(num)) => tokens.push(Token::Number(num)),
            _ => {
                return Err((
                    "unexpected token here (context: value)".to_owned(),
                    lexer.span(),
                ))
            }
        };
    }

    Ok(tokens)
}
