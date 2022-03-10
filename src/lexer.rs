use crate::token::TokenType;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a>
{
    input : Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a>{
    pub fn new(input : &str) -> Lexer {
        Lexer { input : input.chars().peekable() }
    }

    pub fn next_token(&mut self) -> TokenType{
        self.skip_whitespaces();

        match self.read_char() {
            Some('=') => TokenType::Equal,
            Some(ch) => {
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    TokenType::check_ident(&literal)
                }
                else if ch.is_numeric(){
                    TokenType::Integer(self.read_number(ch))
                }
                else {
                    TokenType::Illegal
                }
            },
            None => TokenType::Eof,
        }
    }

    fn peek_char(&mut self) -> Option<&char>{
        self.input.peek()
    }
    
    fn skip_whitespaces(&mut self){
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn read_number(&mut self, first : char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&ch) = self.peek_char(){
            if !ch.is_numeric(){
                break;  
            }
            number.push(self.read_char().unwrap());
        }

        number
    }

    fn read_identifier(&mut self, first : char) -> String{
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap());
        }
        
        ident
    }

    fn peek_is_letter(&mut self) -> bool{
        match self.peek_char() {
            Some(&ch) => is_letter(ch) || ch.is_numeric(),
            None => false
        }
    }
}

fn is_letter(ch: char) -> bool{
    ch.is_alphabetic() || ch == '_'
}