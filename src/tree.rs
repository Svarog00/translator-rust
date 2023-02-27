use std::{rc::Rc, collections::VecDeque, cell::{RefCell}, borrow::BorrowMut};

use crate::token::*;
use crate::prelude::*;

pub struct AstTree {
    token_list : VecDeque<TokenType>,
    space_counter : u32,
    current_token : TokenType,
    previous_token : TokenType,

    tmp_name : String,
    tmp_type : String,

    struct_types : Vec<String>,

    root : Rc<RefCell<TreeNode>>,
    current : Rc<RefCell<TreeNode>>,
}

impl AstTree {
    pub fn new(new_token_list : VecDeque<TokenType>, struct_types : Vec<String>) -> Self {
        AstTree { 
            token_list : new_token_list,
            space_counter : 0,

            current_token : TokenType::default(),
            previous_token : TokenType::default(),

            tmp_name : String::new(),
            tmp_type : String::new(),

            struct_types,

            root : Rc::new(RefCell::new(TreeNode::new())),
            current : Rc::new(RefCell::new(TreeNode::new())),
        }
    }

    pub fn write_out(&mut self) {
        self.current = self.root.clone();
        println!("Program");
        (*self.current).borrow_mut().value = Some(TokenType::Program);
        
        self.space_counter += 1;
        self.out_spaces();
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::Identifier(name) => {
                    self.tmp_type = name.clone();
                    self.space_counter += 1;
                    self.declare();

                    self.space_counter -= 1;
                }
                TokenType::Type(name) => {
                    self.tmp_type = name.clone();
                    self.space_counter += 1;       
                    self.declare();

                    self.space_counter -= 1;
                }
                TokenType::Struct => {
                    println!("StructDeclare");
                    (*self.current).borrow_mut().add_child(
                        Rc::new(RefCell::new(TreeNode::new_init(
                            TokenType::StructDecalre, Rc::clone(&self.current))))
                    );
                    self.space_counter += 1;
                    self.out_spaces();
                    self.struct_declare();

                    self.get_to_parent();

                    self.space_counter -= 2;
                }
                TokenType::Eof => {
                    break;
                }
                _ => {
                    continue;
                } 
            }
        }
    }

    fn struct_declare(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) => {
                println!("{}", name);
                self.space_counter += 1;
                self.struct_body();

                self.space_counter -= 1;
            }
            _ => {}
        }
    }

    fn struct_body(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::Type(name) => {
                    self.tmp_type = name.clone();
                    self.declare();
                }
                TokenType::OpenningBrace => {
                    continue;
                }
                TokenType::ClosingBrace => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn declare(&mut self){
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) => {
                self.tmp_name = name.clone();
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        self.out_spaces();
                        println!("VarDeclare");
                        self.add_child(TokenType::VarDeclare);
                        //Get to children
                        let current_clone = self.current.clone();
                        self.current = (*current_clone).borrow_mut().children[0].clone();

                        self.space_counter+=1;
                        self.var();

                        self.get_to_parent();
                        self.space_counter-=1;
                    }
                    TokenType::OpenningParenthesis => {
                        self.out_spaces();
                        println!("FunctionDeclare");
                        
                        self.add_child(TokenType::FunctionDeclare);

                        let current_clone = self.current.clone();
                        self.current = (*current_clone).borrow_mut().children[0].clone();  

                        self.space_counter+=1;
                        self.out_spaces();
                        println!("{}", self.tmp_type); 
                        self.add_child(TokenType::Type(self.tmp_type.clone()));

                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        self.add_child(TokenType::Identifier(self.tmp_type.clone()));

                        self.function_params();
                        
                        self.next_token();
                        match self.current_token {
                            TokenType::Semicolon => {
                                return;
                            }
                            TokenType::OpenningBrace => {
                                self.space_counter+=1;
                                self.function_body();
                                self.space_counter+=1;
                            }
                            _ => {},
                        }
                        //Tree up
                        self.space_counter-=1;
                    }
                    TokenType::OpenningArray => {
                        self.out_spaces();
                        println!("VarDeclare");
                        self.add_child(TokenType::VarDeclare);
                        //Get to children
                        let current_clone = self.current.clone();
                        self.current = (*current_clone).borrow_mut().children[0].clone();

                        self.space_counter+=1;
                        self.var();
                        self.space_counter+=1;
                        self.array_count_element();

                        self.next_token();
                        //Tree up
                        self.space_counter-=2;
                    }
                    TokenType::Assign => {
                        self.out_spaces();
                        println!("VarDeclare");
                        self.add_child(TokenType::VarDeclare);
                        //Get to children
                        let current_clone = self.current.clone();
                        self.current = (*current_clone).borrow_mut().children[0].clone();

                        self.space_counter+=1;
                        self.var();
                        self.out_spaces();
                        println!("{:?}", self.current_token);
                        self.add_child(self.current_token.clone());
                        self.assign();
                        self.get_to_parent();
                        self.space_counter-=1;
                    }
                    _ => {},
                }
            }
            _ => {}
        }
    }

    fn var(&mut self) {
        self.out_spaces();
        println!("{}", self.tmp_name);
        self.add_child(TokenType::Identifier(self.tmp_type.clone()));

        self.out_spaces();
        println!("{}", self.tmp_type); 
        self.add_child(TokenType::Type(self.tmp_type.clone()));
    }

    fn function_params(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::Type(name) | TokenType::Identifier(name) => {
                    //println!("{:?}", self.current_token);
                    self.tmp_type = name.clone();
                    self.space_counter+=1;

                    self.next_token();
                    match &self.current_token {
                        TokenType::Identifier(name) => {
                            self.tmp_name = name.clone();
                            self.var();
                        }
                        _ => {}
                    }

                    self.space_counter-=1;
                }
                TokenType::ClosingParenthesis => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn function_args(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::ClosingParenthesis => {
                return;
            }
            TokenType::Identifier(name) => {
                self.tmp_name = name.clone();
                self.primary();
                match self.current_token {
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.space_counter+=1;
                        self.expression();
                        self.space_counter-=1;
                    },
                    TokenType::ClosingParenthesis => {
                        return;
                    }
                    _ => {}
                }
                match self.current_token {
                    TokenType::Comma => {
                        self.function_args();
                    }
                    TokenType::ClosingParenthesis => {
                        return;
                    }
                    _ => {}
                }
            }
            TokenType::Number(name) | TokenType::Bool(name) =>{
                self.tmp_name = name.clone();
                self.next_token();
                match self.current_token {
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    TokenType::ClosingParenthesis => {
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        return;
                    }
                    TokenType::Comma => {
                        println!("{}", self.tmp_name);
                        self.function_args();
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.next_token();
        match self.current_token {
            TokenType::Comma => {
                self.function_args();
            }
            _ => {},
        }
    }

    fn array_count_element(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) | TokenType::Number(name) => {
                self.out_spaces();
                println!("{}", name);
                self.next_token();
                match self.current_token {
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    TokenType::ClosingArray => {
                        return;
                    }
                    _ => {}
                }
            }
            TokenType::ClosingArray => {
                return;
            }
            _ => {}
        }
    }

    fn array_access_element(&mut self) {
        self.next_token(); 
        match &self.current_token {
            TokenType::Identifier(name) | TokenType::Number(name) => {
                self.space_counter+=1;
                self.out_spaces();
                println!("{}", name);
                self.next_token();
                match self.current_token {
                    TokenType::ClosingArray => {
                        self.space_counter-=1;
                        return;
                    }
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    _ => {}
                }
                self.space_counter-=1;
            }
            _ => {}
        }
    }

    fn expression(&mut self) {
        self.out_spaces();
        println!("{:?}", self.current_token);
        self.next_token();
        match self.current_token {
            TokenType::OpenningParenthesis => {
                self.next_token();
                self.space_counter+=1;
            }
            _=> {},
        }

        match &self.current_token {
            TokenType::Number(name) | TokenType::Bool(name) => {
                self.space_counter+=1;
                self.out_spaces();
                println!("{}", name);
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        return;
                    }
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    _ => {}
                }
                self.space_counter-=1;
            }
            TokenType::Identifier(name) => {
                self.tmp_name = name.clone();
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        self.space_counter+=1;
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        self.space_counter-=1;
                        return;
                    }
                    TokenType::Dot => {
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        self.space_counter+=1;
                        self.struct_access();
                        self.space_counter-=1;
                    }
                    TokenType::OpenningArray => {
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        self.array_access_element();
                    }
                    TokenType::OpenningParenthesis => {
                        self.function_args();
                    }
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        match self.current_token {
            TokenType::Semicolon => {
                return;
            }
            TokenType::Equal | TokenType::NotEqual | 
            TokenType::GreaterOrEqual | TokenType::GreaterThan |
            TokenType::LowerThan | TokenType::LowerOrEqual => {
                return;
            }
            _ => {}
        }
        self.next_token();
        match self.current_token {
            TokenType::ClosingParenthesis => {
                self.space_counter-=1;
            }
            TokenType::Multi | TokenType::Plus | 
            TokenType::Minus | TokenType::Divide => {
                self.space_counter+=1;
                self.expression();
                self.space_counter-=1;
            },
            _ => {},
        }
    }

    fn condition(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::ClosingParenthesis => {
                return;
            }
            TokenType::Identifier(name) => {
                self.tmp_name = name.clone();
                self.primary();
                match self.current_token {
                    TokenType::Equal | TokenType::NotEqual | 
                    TokenType::GreaterOrEqual | TokenType::GreaterThan |
                    TokenType::LowerThan | TokenType::LowerOrEqual => {
                        self.space_counter+=1;
                        self.equation();
                        self.space_counter-=1;
                    }
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.space_counter+=1;
                        self.expression();
                        self.space_counter-=1;
                    },
                    TokenType::And | TokenType::Or => {
                        self.out_spaces();
                        println!("{:?}", self.current_token);
                        self.condition();
                    }
                    TokenType::ClosingParenthesis => {
                        return;
                    }
                    _ => {}
                }
                
                match self.current_token {
                    TokenType::Equal | TokenType::NotEqual | 
                    TokenType::GreaterOrEqual | TokenType::GreaterThan |
                    TokenType::LowerThan | TokenType::LowerOrEqual => {
                        self.space_counter+=1;
                        self.equation();
                        self.space_counter-=1;
                    }
                    TokenType::And | TokenType::Or => {
                        self.out_spaces();
                        println!("{:?}", self.current_token);
                        self.condition();
                    }
                    TokenType::ClosingParenthesis => {
                        return;
                    }
                    _ => {}
                }
            }
            TokenType::Number(name) | TokenType::Bool(name) =>{
                self.tmp_name = name.clone();
                self.next_token();
                match self.current_token {
                    TokenType::Equal | TokenType::NotEqual | 
                    TokenType::GreaterOrEqual | TokenType::GreaterThan |
                    TokenType::LowerThan | TokenType::LowerOrEqual => {
                        self.equation();
                    }
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    TokenType::ClosingParenthesis => {
                        return;
                    }
                    _ => {}
                }

                match self.current_token {
                    TokenType::Equal | TokenType::NotEqual | 
                    TokenType::GreaterOrEqual | TokenType::GreaterThan |
                    TokenType::LowerThan | TokenType::LowerOrEqual => {
                        self.space_counter+=1;
                        self.equation();
                        self.space_counter-=1;
                    }
                    TokenType::And | TokenType::Or => {
                        self.condition();
                    }
                    TokenType::ClosingParenthesis => {
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn equation(&mut self) {
        self.out_spaces();
        println!("{:?}", self.current_token);
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) => {
                self.tmp_name = name.clone();
                self.primary();
                match self.current_token {
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.space_counter+=1;
                        self.expression();
                        self.space_counter-=1;
                    },
                    _ => { 
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        return;
                    }
                }
            }
            TokenType::Number(name) | TokenType::Bool(name) => {
                self.tmp_name = name.clone();
                self.next_token();
                match self.current_token {
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.space_counter+=1;
                        self.expression();
                        self.space_counter-=1;
                    },
                    _ => { 
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        return;
                    }
                }
            }
            _ => {}
        }
    }

    fn assign(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) | TokenType::Number(name) | TokenType::Bool(name) => {
                self.tmp_name = name.clone();
                self.next_token();
                match self.current_token {
                    TokenType::Semicolon => {
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        return;
                    }
                    TokenType::Dot => {
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        self.space_counter+=1;
                        self.struct_access();
                        self.space_counter-=1;
                    }
                    TokenType::OpenningArray => {
                        self.out_spaces();
                        println!("{}", self.tmp_name);
                        self.space_counter+=1;
                        self.array_access_element();
                        self.space_counter-=1;
                    }
                    TokenType::OpenningParenthesis => {
                        self.function_args();
                    }
                    TokenType::Multi | TokenType::Plus | 
                    TokenType::Minus | TokenType::Divide => {
                        self.expression();
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        match self.current_token {
            TokenType::Multi | TokenType::Plus | 
            TokenType::Minus | TokenType::Divide => {
                self.space_counter+=1;
                self.expression();
                self.space_counter-=1;
            },
            _ => {},
        }
    }

    fn if_out(&mut self) {
        self.out_spaces();
        println!("If");
        self.next_token();
        match self.current_token {
            TokenType::OpenningParenthesis => {
                self.space_counter+=1;
                self.condition();
                self.space_counter-=1;
            }
            _ => {}
        }
        self.next_token();
        match self.current_token {
            TokenType::OpenningBrace => {
                self.space_counter+=1;
                self.function_body();
                self.space_counter-=1;
            }
            _ => {}
        }
    }

    fn while_out(&mut self) {
        self.out_spaces();
        println!("While");
        self.next_token();
        match self.current_token {
            TokenType::OpenningParenthesis => {
                self.space_counter+=1;
                self.condition();
                self.space_counter-=1;
            }
            _ => {}
        }
        self.next_token();
        match self.current_token {
            TokenType::OpenningBrace => {
                self.space_counter+=1;
                self.function_body();
                self.space_counter-=1;
            }
            _ => {}
        }
    }

    fn struct_access(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Identifier(name) => {
                self.out_spaces();
                self.tmp_name = name.clone();
                println!("{}", self.tmp_name);
                self.next_token();
                if self.current_token == TokenType::Dot {
                    self.space_counter+=1;
                    self.struct_access();
                    self.space_counter-=1;
                }
                self.space_counter-=1;
            },
            _ => return,
        }
    }

    fn primary(&mut self) {
        self.next_token();
        match &self.current_token {
            TokenType::Dot => {
                self.out_spaces();
                println!("{}", self.tmp_name);
                self.space_counter+=1;
                self.struct_access();
                self.space_counter-=1;
            }
            TokenType::OpenningArray => {
                self.out_spaces();
                println!("{}", self.tmp_name);
                self.space_counter+=1;
                self.array_access_element();
                self.space_counter-=1;
            }
            TokenType::Assign => {
                self.out_spaces();
                println!("{}", self.tmp_name);
                self.out_spaces();
                println!("{:?}", self.current_token);
                self.space_counter+=1;
                self.assign();
                self.space_counter-=1;
            }
            TokenType::OpenningParenthesis => {
                self.out_spaces();
                println!("{}", self.tmp_name);
                self.space_counter+=1;
                self.function_args();
                self.space_counter-=1;
            }
            _ => {
                self.out_spaces();
                println!("{}", self.tmp_name);
            }
        }
    }

    fn function_body(&mut self) {
        loop {
            self.next_token();
            match &self.current_token {
                TokenType::ClosingBrace => {
                    break;
                }
                TokenType::Type(name) => {
                    self.tmp_type = name.clone();
                    self.declare();
                }
                TokenType::Identifier(name) => {
                    if self.struct_types.contains(name) {
                        self.tmp_type = name.clone();
                        self.declare();
                    }
                    else {
                        self.tmp_name = name.clone();
                        self.primary();
                    }
                }
                TokenType::Return => {
                    self.return_out();
                }

                TokenType::If => {
                    self.if_out();
                }
                TokenType::While => {
                    self.while_out();
                }
                _ => {} 
            }
        }
    }

    fn return_out(&mut self) {
        self.next_token();
        match self.current_token {
            _ => return,
        }
    }

    fn next_token(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.token_list.pop_front().unwrap();
    }

    fn out_spaces(&self) {
        let mut i = 0;
        while i < self.space_counter {
            print!(" ");
            i+=1;
        }
    }

    fn get_to_parent(&mut self) {
        let current_clone = Rc::clone(&self.current);
        self.current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
    }

    fn add_child(&mut self, token : TokenType){
        (*self.current).borrow_mut().add_child(
            Rc::new(RefCell::new(TreeNode::new_init(
                token, Rc::clone(&self.current))))
        );
    }
}