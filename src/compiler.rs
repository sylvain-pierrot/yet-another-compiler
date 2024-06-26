use logos::Span;

use crate::tokenizer::tokenizer;

type Error = (String, Span);
type Result<T> = std::result::Result<T, Error>;

pub fn compiler(input: String) -> Result<()> {
    // 1. Lexical Analysis -
    //      Breaks the input code (string) into the basic syntax
    //      of the language (array of objects)
    let tokens = tokenizer(input)?;
    println!("{:?}", tokens);
    // 2. Syntactic Analysis
    //      Transforms the tokens (array of objects) into an
    //      AST (tree of objects) which represents our program

    // 3. Transformation
    // 4. Code Generation
    //
    // return rustCode;

    Ok(())
}
