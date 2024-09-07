use lexer::Lexer;

mod lexer;
mod parser;

const SOURCE: &str = include_str!("../example.th");

fn main() {
    let mut lexer = Lexer::new(SOURCE);

    let tokens = lexer.tokens();

    lexer.print_tokens(&tokens);
}
