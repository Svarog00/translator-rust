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
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char(); 
                    TokenType::Equal
                }
                else {
                    TokenType::Assign
                }
            }
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    TokenType::NotEqual
                }
                else {
                    TokenType::Not
                }
            }
            Some('&') => {
                if self.peek_char_eq('&') {
                    self.read_char();
                    TokenType::And
                }
                else {
                    TokenType::Illegal
                }
            }
            Some('.') => TokenType::Dot,
            Some(',') => TokenType::Comma,            
            Some('+') => TokenType::Plus,
            Some('-') => TokenType::Minus,
            Some(';') => TokenType::Semicolon,
            Some('{') => TokenType::OpenningBrace,
            Some('}') => TokenType::ClosingBrace,
            Some('(') => TokenType::OpenningParenthesis,
            Some(')') => TokenType::ClosingParenthesis,
            Some('[') => TokenType::OpenningArray,
            Some(']') => TokenType::ClosingArray,
            Some('>') => {
                if self.peek_char_eq('='){
                    self.read_char();
                    TokenType::GreaterOrEqual
                }
                else {
                    TokenType::GreaterThan
                }
            }
            Some('<') => {
                if self.peek_char_eq('='){
                    self.read_char();
                    TokenType::LowerOrEqual
                }
                else {
                    TokenType::LowerThan
                }
            }
            Some(ch) => {
                if is_letter(ch) {
                    let literal = self.read_literal(ch);
                    TokenType::check_identifier(&literal)
                }
                else if ch.is_numeric() {
                    let literal = self.read_literal(ch);
                    TokenType::check_number(&literal)
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

    fn peek_char_eq(&mut self, ch : char) -> bool{
        match self.peek_char(){
            Some(&peeked_ch) => peeked_ch == ch,
            None => false
        }
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
            if ch.is_digit(10) || ch == '.'{
                number.push(self.read_char().unwrap());
            }
            else {
                break;  
            }
        }

        number
    }

    fn read_literal(&mut self, first : char) -> String{
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_legal() {
            ident.push(self.read_char().unwrap());
        }
        
        ident
    }

    fn peek_is_legal(&mut self) -> bool{
        match self.peek_char() {
            Some(&ch) => is_letter(ch) || ch.is_numeric(),
            None => false
        }
    }
}

fn is_letter(ch: char) -> bool{
    ch.is_alphabetic() || ch == '_'
}