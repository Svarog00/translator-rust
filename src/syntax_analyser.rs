use crate::prelude::*;
use std::{vec::*, collections::VecDeque};

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
    previous_token : TokenType,
    current_token : TokenType,
    pub struct_types : Vec<String>,
    openning_parenthesis_counter : i32,
    closing_parenthesis_counter : i32,

    token_vec : VecDeque<TokenType>,
}

impl<'a> Analyser<'a> {
    pub fn new(lexer : Lexer<'a>) -> Self{
        Analyser {
            lexer,
            current_token : TokenType::default(),
            previous_token : TokenType::default(),
            struct_types : Vec::new(),
            openning_parenthesis_counter : 0,
            closing_parenthesis_counter : 0,

            token_vec : VecDeque::<TokenType>::new(),
        }
    }

    pub fn start_analysis(&mut self) -> VecDeque<TokenType> {
        self.next_token();
        while self.current_token != TokenType::Eof {
            match &self.current_token {
                TokenType::Type( name ) => {
                    self.check_declare();
                },
                TokenType::Struct => {
                    self.check_struct_declare();
                },
                TokenType::Identifier( name ) => {
                    if self.struct_types.contains(&(name.clone())) {

                        self.check_declare();
                    }
                    else {
                        self.panic_syntax_error("Wrong start token");
                    }
                }
                _ => {
                    self.panic_syntax_error("Wrong start token");
                    return VecDeque::new();
                },
            }

            self.next_token();
            if self.current_token == TokenType::Semicolon {
                self.next_token();
            }
        }


        self.token_vec.clone()
    }

    fn panic_syntax_error(&self, message : &str) {
        panic!("{}", message);
    }

    fn next_token(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.lexer.next_token();
        self.token_vec.push_back(self.current_token.clone());
        println!("Parser got toke: {:?}", self.current_token);
    }

    fn check_declare(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier( name ) => {
                self.next_token();
                match self.current_token {
                    TokenType::OpenningParenthesis => {
                        self.check_func_params();
                        self.next_token();
                        match self.current_token {
                            TokenType::OpenningBrace => {
                                self.check_statement();
                            },
                            TokenType::Semicolon => {
                                return
                            },
                            _ => self.panic_syntax_error("After func declaration must go block or semicolon"),
                        }
                    },
                    TokenType::Semicolon => {
                        return;
                    },
                    TokenType::OpenningArray => {
                        self.check_array_declare();
                    },
                    TokenType::Assign => {
                        self.check_assign();
                    },
                    _ => self.panic_syntax_error("After id in declaration should go assign, array assign, semicolon or func params"),
                }
            },
            _ => self.panic_syntax_error("After type declaration should go identifier"),
        }
    }
    
    fn check_func_params(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::Type( name ) => {
                    self.check_var();
                    self.next_token();
                    match self.current_token {
                        TokenType::Comma => continue,
                        TokenType::ClosingParenthesis => break,
                        _ => self.panic_syntax_error("Wrong token after one of func params"),
                    }
                },
                TokenType::ClosingParenthesis => break,
                _ => self.panic_syntax_error("In function params wrong token"),
            }
        }
    }

    fn check_var(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) => {
                //Add var node in AST
            },
            _ => self.panic_syntax_error("After type declaration must go identifier"),
        }
    }

    fn check_array_declare(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Number(name) => {
                //Add array node with number in count

                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Multi | 
                    TokenType::Minus | TokenType::Divide => {
                        self.check_expression();
                    }
                    TokenType::ClosingArray => {
                        return;
                    }
                    _ => self.panic_syntax_error("Wrong token in array declaration"),
                }
            }
            TokenType::ClosingArray => {
                //Add array node with 0 in count
                return;
            }
            TokenType::Identifier(name) =>{
                //Add array node with identifier in count
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Multi | 
                    TokenType::Minus | TokenType::Divide => {
                        self.check_expression();
                    }
                    TokenType::ClosingArray => {
                        return;
                    }
                    _ => self.panic_syntax_error("Wrong token in array declaration"),
                }
            }
            _ => self.panic_syntax_error("In array declaration expected number, identifier or nothing"),
        }
        if self.current_token == TokenType::ClosingArray {
            return;
        }
        else {
            self.panic_syntax_error("Expected closing array");
        }
    }

    fn check_statement(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::Semicolon => continue,
                TokenType::ClosingBrace => break,
                TokenType::Identifier( name ) => {
                    if self.struct_types.contains(name) {
                        self.check_declare();
                    } 
                    else {
                        self.check_primary();
                    }
                },
                TokenType::If => {
                    self.check_if_state();
                    if self.current_token == TokenType::ClosingBrace {
                        return;
                    }
                },
                TokenType::While => {
                    self.check_while_state();
                },
                TokenType::Type(_) => {
                    self.check_declare();
                    self.next_token();
                },
                TokenType::Return => {
                    self.check_return();
                },
                _ => {
                    self.panic_syntax_error("Unexpected token in statement body");               
                }           
            }
        }
    }

    fn check_while_state(&mut self) {   
        self.next_token();
        match self.current_token {
            TokenType::OpenningParenthesis => {
                self.check_condition();
            }
            _ => self.panic_syntax_error("Expected condition after if"),
        }
        //self.check_condition();
        self.next_token();
        match self.current_token {
            TokenType::OpenningBrace => self.check_statement(),
            _ => self.panic_syntax_error("After while expected body"),
        }
    }

    fn check_while_statement(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::ClosingBrace => break,
                TokenType::Identifier( name ) => {
                    if self.struct_types.contains(name) {
                        self.check_declare();
                    }
                    else {
                        self.check_primary();
                    }
                },
                TokenType::If => {
                    self.check_if_state();
                    if self.current_token == TokenType::ClosingBrace {
                        return;
                    }
                },
                TokenType::While => {
                    self.check_while_state();
                },
                TokenType::Type(_) => {
                    self.check_declare();
                    self.next_token();
                },
                TokenType::Return => {
                    self.check_return();
                },
                TokenType::Break | TokenType::Continue | TokenType::Semicolon => {
                    continue;
                }
                _ => {
                    self.panic_syntax_error("Unexpected token in statement body");                
                }           
            }
        }
    }

    fn check_return(&mut self){
        self.next_token();
        match &self.current_token {
            TokenType::Number( name ) | TokenType::Identifier( name ) | TokenType::Bool( name ) => {
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Minus | 
                    TokenType::Multi | TokenType::Divide => {
                        //Add expression node
                        self.check_expression();
                    },
                    TokenType::Equal | TokenType::NotEqual | 
                    TokenType::GreaterOrEqual | TokenType::GreaterThan |
                    TokenType::LowerThan | TokenType::LowerOrEqual => {
                        self.check_equation();
                    }
                    TokenType::Semicolon => {
                        return;
                    }
                    _ => self.panic_syntax_error("Wrong token after return expression"),
                }
            }
            _ => self.panic_syntax_error("Wrong token after return word"),
        }

        match self.current_token {
            TokenType::Semicolon => return,
            _ => self.panic_syntax_error("Expected semicolon after return expression")
        }
    }

    fn check_if_state(&mut self) {
        self.next_token();
        match self.current_token {
            TokenType::OpenningParenthesis => {
                self.check_condition();
            }
            _ => self.panic_syntax_error("Expected condition after if"),
        }

        //self.check_condition();
        self.next_token();
        match self.current_token {
            TokenType::OpenningBrace => self.check_statement(),
            _ => self.panic_syntax_error("After if expected body"),
        }
        self.next_token();
        if self.current_token == TokenType::Else {
            self.next_token();
            match self.current_token {
                TokenType::OpenningBrace => {
                    self.check_statement();
                },
                TokenType::If => {
                    self.check_if_state();
                }
                _ => self.panic_syntax_error("After else expected body or if"),
            }
        }
        else {
            return;
        } 
    }

    fn check_primary(&mut self) {
        self.next_token();
        loop {
            match self.current_token {
                TokenType::Assign => {
                    //Add assign node
                    self.check_assign();
                },
                TokenType::OpenningArray => {
                    //Add array access node
                    self.check_array_access_element();
                    self.next_token();
                    
                }, 
                TokenType::OpenningParenthesis => {
                    //Add func call node (id, node args (id, num list))
                    self.check_func_args();
                    self.next_token();
                },
                TokenType::Dot => {
                    //add struct access node
                    self.check_struct_access();
                    continue;
                },
                _ => self.panic_syntax_error("Wrong token after identifier in statement"),
            }

            //self.next_token();
            match self.current_token {
                TokenType::Semicolon => break,
                _ => {
                    self.panic_syntax_error("Expected semicolon after primary in statement")
                }
            }
        }
    }

    fn check_assign(&mut self) {
        self.next_token();

        match &self.current_token {
            TokenType::Identifier(name) => {
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        //Add in assign node this token type
                        return;
                    },
                    TokenType::Plus | TokenType::Minus | 
                    TokenType::Multi | TokenType::Divide => {
                        //Add expression node
                        self.check_expression();
                    },
                    TokenType::OpenningParenthesis => {
                        //add func call node
                        self.check_func_args();
                    },
                    TokenType::Dot => {
                        self.check_struct_access();
                    }
                    TokenType::OpenningArray => {
                        self.check_array_access_element();
                    }
                    _ => self.panic_syntax_error("In assign occurred error"),
                }
            },
            TokenType::Number(name) => {
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        //Add in assign node this token type
                        return;
                    },
                    TokenType::Plus | TokenType::Minus | 
                    TokenType::Multi | TokenType::Divide => {
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

        //self.next_token();
        match self.current_token {
            TokenType::Plus | TokenType::Minus | 
            TokenType::Multi | TokenType::Divide => {
                //Add expression node
                self.check_expression();
            },
            _ => return,
        }
    }

    fn check_expression(&mut self) {
        self.next_token();

        match self.current_token{
            TokenType::OpenningParenthesis => {
                self.openning_parenthesis_counter += 1;
                self.next_token();
            }
            _ => {}
        }

        match &self.current_token {
            TokenType::Number(name) | TokenType::Bool(name)=> {
                //add this token to expression node
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Minus | 
                    TokenType::Multi | TokenType::Divide => {
                        //Add expression node
                        self.check_expression();
                    },
                    _ => return,
                }
            },
            TokenType::Identifier(name) => {
                //add this token to expression node
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Minus | 
                    TokenType::Multi | TokenType::Divide => {
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
                        self.next_token();
                    },
                    TokenType::OpenningParenthesis => {
                        //Add fuction call node
                        self.check_func_args();
                    },
                    _ => return,
                }
            },
            _ => self.panic_syntax_error("In expression wrong token")
        }

        //self.next_token();
        match self.current_token {
            TokenType::ClosingParenthesis => {
                self.closing_parenthesis_counter+=1;
                if self.closing_parenthesis_counter != self.openning_parenthesis_counter {
                    self.panic_syntax_error("Explicit parenthesis")
                }
                self.next_token();
            }
            _ => {}
        }
        
        match self.current_token {
            TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Divide => {
                //Add expression node
                self.check_expression();
            },
            _ => return,
        }
    }


    fn check_condition(&mut self) {
        //self.next_token();
        //match self.current_token {
        //    TokenType::OpenningParenthesis => {
                //add condition node
                //loop {
                    self.next_token();
                    match &self.current_token {
                        TokenType::Identifier(name) | TokenType::Number(name) 
                        | TokenType::Bool(name) => {

                            self.next_token();
                            match self.current_token {
                                TokenType::Equal | TokenType::NotEqual | 
                                TokenType::GreaterOrEqual | TokenType::GreaterThan |
                                TokenType::LowerThan | TokenType::LowerOrEqual => {
                                    self.check_equation();
                                }
                                TokenType::Plus | TokenType::Minus | 
                                TokenType::Multi | TokenType::Divide => {
                                    self.check_expression();
                                }
                                TokenType::And | TokenType::Or => {
                                    //add bin operator in tree
                                    self.check_condition();
                                    //continue;
                                }
                                TokenType::ClosingParenthesis => return,
                                _ => self.panic_syntax_error("After id expected cl. parenthesis, logic op or math. sign"),
                            }
                            
                            match self.current_token {
                                TokenType::ClosingParenthesis => return,
                                TokenType::And | TokenType::Or => {
                                    //add bin operator in tree
                                    self.check_condition();
                                    //continue
                                }
                                TokenType::Equal | TokenType::NotEqual | 
                                TokenType::GreaterOrEqual | TokenType::GreaterThan |
                                TokenType::LowerThan | TokenType::LowerOrEqual => {
                                    self.check_equation();
                                }
                                _ => self.panic_syntax_error("Expected logic op or closing parenth"),
                            }
                        },
                        TokenType::ClosingParenthesis => {
                            return;
                        },
                        _ => self.panic_syntax_error("Expected closing parenth, number, bool or id"),
                    }
                }
            //}
    //        _ => self.panic_syntax_error("Expected openning parenthesis after if/while"),
    //    }
    //}

    fn check_equation(&mut self) {
        //add new equation node

        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) | TokenType::Number(name) | TokenType::Bool(name)=> {
                self.next_token();
                match self.current_token {
                    TokenType::Plus | TokenType::Minus | 
                    TokenType::Multi | TokenType::Divide => {
                        //add bin operation node

                        self.check_expression();
                    },
                    _ => return,
                }
            },
            _ => self.panic_syntax_error("Wrong token in equation"),
        }
    }

    fn check_array_access_element(&mut self) {
        self.next_token();
        match &self.current_token {
            //Add array access
            TokenType::Identifier(name) | TokenType:: Number(name) => {
                
                self.next_token();
                match self.current_token {
                    TokenType::ClosingArray => {
                        return;
                    },
                    TokenType::Multi | TokenType::Plus | TokenType::Minus | TokenType::Divide => {
                        self.check_expression();
                    },
                    _ => self.panic_syntax_error("Expected expression or closing array bracket"),
                }
            },
            _ => self.panic_syntax_error("In array access expected number, id or expression"),
        }
    }

    fn check_func_args(&mut self) {
        self.next_token();
        loop {
            match &self.current_token {
                TokenType::Identifier(name) | TokenType::Number(name) => {
                    //Add id or number in node
                    
                    self.next_token();
                    match self.current_token {
                        TokenType::Multi | TokenType::Plus | 
                        TokenType::Minus | TokenType::Divide => {
                            self.check_expression();
                        },
                        TokenType::Comma => {
                            self.next_token();
                            continue;
                        }
                        TokenType::ClosingParenthesis => break,
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

    fn check_struct_declare(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier( identifier ) => {

                self.struct_types.push(identifier.to_string());

                self.next_token();
                match self.current_token {
                    TokenType::OpenningBrace => {
                        self.check_struct_body();
                    },
                    _ => self.panic_syntax_error("After struct name expected '{'"),
                }
            },
            _ => self.panic_syntax_error("After struct word should go id"),
        }
    }

    fn check_struct_body(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::Identifier( name ) => {
                    if self.struct_types.contains(name) {
                        
                        self.check_declare();
                    }
                    else {

                        self.check_primary();
                    }
                },
                TokenType::Type(name) => {
                    self.check_var();
                    self.next_token();
                    match self.current_token {
                        TokenType::Semicolon => continue,
                        _ => self.panic_syntax_error("Wrong token on struct declaration"),
                    }
                },
                TokenType::ClosingBrace => break,
                _ => self.panic_syntax_error("Wrong token on struct declaration"),
            }
        }
    }

    fn check_struct_access(&mut self) {
        //add struct field access
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) => {
                self.next_token();
                if self.current_token == TokenType::Dot {
                    self.check_struct_access();
                }
            },
            _ => return,
        }
    }
    //Function for every poopoo
}