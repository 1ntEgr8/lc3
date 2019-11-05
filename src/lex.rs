use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    INSTR,
    REG,
    PSEUDO(String),
    COMMENT(String)
}

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>
}

impl<'a> Lexer<'a> {
    pub fn new(s: &str) -> Lexer {
        Lexer {
            chars: s.chars().peekable(),
            tokens: Vec::new()
        }
    }

    pub fn lex(&mut self) {
        loop {
            match self.chars.peek() {
                Some(ref ch) => {
                    match ch {
                        ';' => {
                            let comment = self.consume_while(|c| c != '\n');
                            self.tokens.push(Token::COMMENT(comment));                                 
                        },
                        '.' => {
                            let pseudo_op = self.consume_while(|c| !c.is_whitespace());
                            self.tokens.push(Token::PSEUDO(pseudo_op));
                        },
                        _ => {
                            
                        }
                    }
                    self.chars.next();
                },
                None => break
            }
        }
        println!("{:#?}", self.tokens);
    }

    fn consume_while<F>(&mut self, condition: F) -> String
        where F: Fn(char) -> bool {
        let mut s = String::new();

        while self.chars.peek().map_or(false, |c| condition(*c)) {
            s.push(self.chars.next().unwrap());
        }

        s
    }
}