use crate::token::*;
use crate::lexer::*;
use std::vec::*;

pub struct Analyser<'a>{
    lexer : Lexer<'a>,
    current_token : TokenType,
    //logic_tree : Tree,
}

impl<'a> Analyser<'a> {
    pub fn new(lexer : Lexer<'a>) -> Self{
        Analyser {
            lexer,
            current_token : TokenType::default(),
        }
    }

    pub fn start_analysis(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Type(_) => self.check_type(),
            TokenType::Eof => self.eof_token(),
            _ => self.panic_syntax_error("Wrong start token"),
        }
    }

    fn panic_syntax_error(&self, message : &str) {
        panic!("{}", message);
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
        println!("Got token: {:?}", self.current_token);
    }

    fn check_type(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Identifier(_) => self.check_identifier(),
            _ => self.panic_syntax_error("After type declaration should go identifier"),
        }
    }

    fn check_identifier(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Assign => self.check_assign(),
            TokenType::Equal => self.check_equal(),
            TokenType::OpenningParenthesis => self.check_open_parent(),
            TokenType::Semicolon => self.check_semicolon(),
            TokenType::OpenningArray => self.check_open_array(),
            _ => self.panic_syntax_error("Wrong following of identifier"),
        }
    }

    fn check_assign(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Number(_) => self.check_number(),
            TokenType::Identifier(_) => self.check_identifier(),
            _ => self.panic_syntax_error("Wrong following of assign"),
        }
    }

    fn check_equal(&mut self) {
        self.next_token();
        match self.current_token {
            //number, ident
            TokenType::Number(_) => self.check_number(),
            TokenType::Identifier(_) => self.check_identifier(),
            _ => self.panic_syntax_error("After equal can go number or ident"),
        }
    }

    fn check_open_parent(&mut self) {
        self.next_token();
        //type, identifier
    }

    fn check_number(&mut self) {
        self.next_token();
        //semicolon
    }

    fn check_semicolon(&mut self) {
        //Identifier, type, if, while, for
    }

    fn check_open_array(&mut self) {
        //number, identifier
    }

    fn eof_token(&mut self) {
        panic!("End of file.");
    }
    //Function for every poopoo
}