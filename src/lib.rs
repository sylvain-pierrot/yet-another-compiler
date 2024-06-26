use lalrpop_util::lalrpop_mod;

mod ast;
pub mod compiler;
pub mod lexer;
pub mod tokens;

lalrpop_mod!(pub grammar);

#[test]
fn grammar() {
    use lexer::Lexer;

    let input = "(add 2 (sub 4 3))";
    let lexer = Lexer::new(input);
    let expr = grammar::TermParser::new().parse(lexer).unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", expr), "(add 2 (sub 4 3))");
}
