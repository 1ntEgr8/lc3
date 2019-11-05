mod lex;
use std::fs;
use lex::Lexer;

fn main() {
    let text = fs::read_to_string("yeet.asm")
        .expect("Could not read the file");
    
    let mut lexer = Lexer::new(text.as_str());
    lexer.lex();
}
