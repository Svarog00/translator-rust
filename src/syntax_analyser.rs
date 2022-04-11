use crate::token::*;
use crate::lexer::*;
use crate::tree::*;
use std::vec::*;

//statement - внутри {}
//expr - операции с переменными внутри statement
//declare, там смотрим объявляем переменную или функцию.

//check declare получает токены и проверяет последовательно токены, чтобы они были типом и идентификатором. 
//И там добавляем узел func (тип, идентификтор, параметры (var), тело) или var (тип, значение) в зависимости от того, что идет после идентификатора
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
    logic_tree : Tree,
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
                _ => {
                    self.panic_syntax_error("Wrong start token");
                    return false;
                },
            }

            self.next_token();
            if self.current_token == TokenType::Semicolon {
                self.next_token();
            }
        }
        true
    }

    fn panic_syntax_error(&self, message : &str) {
        panic!("{}", message);
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
        println!("Parser got {:?} token", self.current_token);
    }

    fn check_declare(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Identifier(_) => {
                self.next_token();
                match self.current_token {
                    TokenType::OpenningParenthesis => {
                        //Add func node to AST
                        self.check_func_params();
                        self.next_token();
                        match self.current_token {
                            TokenType::OpenningBrace => {
                                self.check_statement();
                            },
                            TokenType::Semicolon => {
                                return
                            }
                            _ => self.panic_syntax_error("After func declaration must go block or semicolon"),
                        }
                    }
                    TokenType::Semicolon => {
                        //Add var node to AST
                        
                    }
                    TokenType::OpenningArray => {
                        self.check_array_declare();
                    }
                    TokenType::Assign => {
                        self.check_assign();
                    }
                    _ => self.panic_syntax_error("After id in declaration should go assign, array assign, semicolon or func params"),
                }
            },
            _ => self.panic_syntax_error("After type declaration should go identifier"),
        }
    }
    
    fn check_func_params(&mut self) {
        loop {
            self.next_token();
            match self.current_token {
                TokenType::Type(_) => {
                    self.check_var();
                    self.next_token();
                    if self.current_token == TokenType::Comma {
                        continue;
                    }
                },
                TokenType::ClosingParenthesis => break,
                _ => self.panic_syntax_error("In function params wrong token"),
            }
        }
    }

    fn check_var(&mut self) {
        match self.current_token {
            TokenType::Type(_) => {
                self.next_token();
                match self.current_token {
                    TokenType::Identifier(_) => {
                        //Add var node in AST
                    }
                    _ => self.panic_syntax_error("After type declaration must go identifier"),
                }
            }
            _ => self.panic_syntax_error("In variable declaration must go first type"),
        }
    }

    fn check_array_declare(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Number(_) => {
                //Add array node with number in count
            }
            TokenType::ClosingArray => {
                //Add array node with 0 in count
            }
            TokenType::Identifier(_) =>{
                //Add array node with identifier in count
            }
            _ => self.panic_syntax_error("In array declaration expected number, identifier or none"),
        }
    }

    fn check_statement(&mut self) {
        loop {
            self.next_token();
            match self.current_token {
                TokenType::ClosingBrace => break,
                TokenType::Identifier(_) => {
                    self.check_primary();
                },
                TokenType::If | TokenType::While | TokenType::For => {
                    self.check_condition();
                    self.check_statement();
                },
                TokenType::Type(_) => {
                    self.check_declare();
                },
                _ => self.panic_syntax_error("Unexpected token in statement body"),
            }

            if self.current_token == TokenType::Semicolon {
                continue;
            }
            else {
                self.panic_syntax_error("Lost semicolon in a statement");
            }
        }
    }

    fn check_primary(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Assign => {
                //Add assign node
                self.check_assign();
            },
            TokenType::OpenningArray => {
                //Add array access token
                self.check_array_access_element();
            }, 
            TokenType::OpenningParenthesis => {
                //Add func call node (id, node args (id, num list))
                self.check_func_args();
            },
            TokenType::Dot => {
                self.check_struct_access();
            },
            _ => self.panic_syntax_error("Wrong token after identifier in statement"),
        }
    }

    fn check_expression(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Number(_) => {
                //add this token to expression node
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Divide => {
                        //Add expression node
                        self.check_expression();
                    },
                    TokenType::Semicolon => {
                        return;
                    }
                    _ => self.panic_syntax_error("In expression after number ocurred problem"),
                }
            },
            TokenType::Identifier(_) => {
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Divide => {
                        //Add expression node
                        self.check_expression();
                    },
                    TokenType::Dot => {
                        //Add struct access node
                        self.check_struct_access();
                    },
                    TokenType::OpenningArray => {
                        //Add array access node
                        self.check_array_access_element();
                    },
                    TokenType::OpenningParenthesis => {
                        //Add fuction call node
                        self.check_func_args();
                    },
                    TokenType::Semicolon => {
                        return;
                    },
                    _ => self.panic_syntax_error("In expression after identifier occurred problem"),
                }
            },
            _ => self.panic_syntax_error("In expression wrong token")
        }
    }

    fn check_assign(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Identifier(_) | TokenType::Number(_) => {
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        //Add in assign node this token type
                    },
                    TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Divide => {
                        //Add expression node
                        self.check_expression();
                    },
                    TokenType::OpenningParenthesis => {
                        //add func call node
                        self.check_func_args();
                    },
                    _ => self.panic_syntax_error("In assign occurred error"),
                }
            },
            _ => self.panic_syntax_error("After assign occurred problem")
        }
    }

    fn check_condition(&mut self) {
        loop {
            self.next_token();
            match self.current_token {
                TokenType::ClosingParenthesis => {
                    break;
                },
                _ => self.panic_syntax_error("Wrong token in condition"),
            }
        }
    }

    fn check_array_access_element(&mut self) {
        self.next_token();
        loop {
            match self.current_token {
                //Add array access
                TokenType::Identifier(_) | TokenType:: Number(_) => {
                    self.next_token();
                    match self.current_token {
                        TokenType::ClosingArray => {
                            return;
                        },
                        TokenType::Multi | TokenType::Plus | TokenType::Multi | TokenType::Divide => {
                            self.check_expression();
                        },
                        _ => self.panic_syntax_error("Expected expression or closing array bracket"),
                    }
                },
                _ => self.panic_syntax_error("In array access expected number, id or expression"),
            }
        }
    }

    fn check_func_args(&mut self) {
        self.next_token();
        loop {
            match self.current_token {
                TokenType::Identifier(_) | TokenType::Number(_) => {
                    //Add id or number in node
                    self.next_token();
                    match self.current_token {
                        TokenType::Comma => continue,
                        _ => self.panic_syntax_error("No comma after arg"),
                    }
                },
                TokenType::ClosingParenthesis => {
                    return;
                },
                _ => self.panic_syntax_error("Error in parsing function args"),
            }
        }
    }

    fn check_struct_access(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::Identifier(_) => {
                return;
            },
            _ => self.panic_syntax_error("Wrong struct element access"),
        }
    }
    //Function for every poopoo
}