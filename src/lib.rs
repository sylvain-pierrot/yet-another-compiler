use lalrpop_util::lalrpop_mod;

pub mod compiler;
pub mod frontend;

lalrpop_mod!(pub grammar);

#[test]
fn grammar() {
    use frontend::lexer::Lexer;

    let input = "(add 2 (sub 4 3))";
    let lexer = Lexer::new(input);
    let expr = grammar::TermParser::new().parse(lexer).unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", expr), "(add 2 (sub 4 3))");
}
