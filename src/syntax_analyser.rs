use crate::token::*;
use crate::lexer::*;
use std::vec::*;

//statement - внутри {}
//expr - операции с переменными внутри statement
//declare, там смотрим объявляем переменную или функцию.

//check declare получает токены и проверяет последовательно токены, чтобы они были типом и идентификатором. 
//И там добавляем узел func (тип, идентификтор, параметры, тело) или var в зависимости от того, что идет после идентификатора
//Если встречаем "(", то пока не встречается ")" мы запускаем check func params. 
//После круглой скобки требуем { и дальше check statement

//Если встречается фигурная скобка - вызываем check statement 
    //В цикле внутри check statement проверям входные токены, пока не встречается "}"
    //в цикле обработка присваиваний при встрече с идентификатором, за которым следует присваивание (check assign) 
    //если открывающаяся скобка - заупускаем check func args, в которой в цикле проверяются идентификаторы, пока не встречается закрывающаяся скобка
    //мат. операции обрабатываются (check experession), которая вызывается внутри check assign,
        //если после идентификатора встретилась мат операция, до встречи с ";"
    //и объявлений при встрече с токеном типа (check declare)

    //Если встречаем if\for\while, вызываем check condition,
        //в котором в цикле вызывается check expression и проверки на логический оператор, пока не встретится закрывающаяся скобка
    //, добавляется нода (идентификатор, аргументы)

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
            TokenType::Type(_) => self.check_declare(),
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

    fn check_declare(&mut self) {
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
        self.next_token();
        match self.current_token {
            TokenType::Identifier(_) => self.check_identifier(),
            TokenType::Type(_) => self.check_declare(),
            TokenType::If => self.check_if(),
            TokenType::While => self.check_while(),
            TokenType::For => self.check_for(),
            TokenType::ClosingBrace => self.check_closing_brace(),
            _ => self.panic_syntax_error("After semicolon cannot go that token"),
        }
        //Identifier, type, if, while, for
    }
    
    fn check_closing_brace(&mut self) {
        //type, identifier
    }

    fn check_if(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::OpenningParenthesis => self.check_open_parent(),
            _ => self.panic_syntax_error("need condition!"),
        }
        //open parent
    }

    fn check_while(&mut self) {
        //open parent
        
    }

    fn check_for(&mut self) {
        //Open parent
    }

    fn check_open_array(&mut self) {
        //number, identifier
    }

    fn eof_token(&mut self) {
        panic!("End of file.");
    }
    //Function for every poopoo
}