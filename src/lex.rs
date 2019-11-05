use std::iter::Peekable;
use std::str::Chars;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Token {
    LABEL,
    INSTR,
    REG,
    PSEUDO(String),
    COMMENT(String),
    NUMBER(NumberData)
}

#[derive(Debug)]
struct NumberData {
    radix: Radix,
    value: String
}

#[derive(Debug)]
enum Radix {
    BIN,
    HEX,
    DEC
}

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>
}

#[derive(Debug)]
struct LexerError {
    message: String
}

impl LexerError {
    fn new(message: &str) -> LexerError {
        LexerError {
            message: String::from(message)
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
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
                            let token = self.lex_token();
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

    fn lex_token(&mut self) -> Result<Token, LexerError> {
        let ch = self.chars.next().unwrap();

        match ch {
            '#' => {
                
            },
            'x' => {

            },
            '0' => {

            },
            _ => {

            }
        }
        Ok(Token::REG)
    }
}