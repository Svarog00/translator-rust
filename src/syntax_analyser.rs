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

    pub fn start_analysis(&mut self) -> bool {
        self.next_token();
        while self.current_token != TokenType::Eof {
            match self.current_token {
                TokenType::Type(_) => {
                    self.check_declare();
                },
                TokenType::Eof => return true,
                _ => {
                    self.panic_syntax_error("Wrong start token");
                    return false;
                },
            }
            self.next_token();
        }
        true
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

    
    //Function for every poopoo
}