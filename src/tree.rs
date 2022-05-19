use std::{ops::{DerefMut, Deref}, rc::Rc, collections::VecDeque};

use crate::token::*;

#[derive(Debug, Clone)]
pub struct Tree {
    root : ExpressionNode,
    previous_nodes : Vec<Rc<ExpressionNode>>,
    current_node : Rc<ExpressionNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: ExpressionNode::ProgramNode { expressions: Vec::new() },
            current_node: Rc::new(ExpressionNode::new()),
            previous_nodes : Vec::new(),
        }
    }

    pub fn add_node_to_current_node(&mut self, expression_node : &ExpressionNode) {
        match self.current_node.deref().to_owned() {
            ExpressionNode::ProgramNode { mut expressions } => {
                expressions.push(Rc::new(expression_node.clone()));

            },
            ExpressionNode::StatementNode { mut nodes } => {
                nodes.push(Rc::new(expression_node.clone()));

            },
            ExpressionNode::BinaryOperation { mut right_operand, .. } => {
                right_operand = Rc::new(expression_node.clone());

            },
            ExpressionNode::ConditionNode { mut nodes } => {
                nodes.push(Rc::new(expression_node.clone()));

            },
            ExpressionNode::CallFunctionNode { mut arguments , .. } => {
                arguments.push(Rc::new(expression_node.clone()));
                
            },
            ExpressionNode::DeclareFunctionNode { identifier, 
                params, body, return_type } => {
                
            },
            ExpressionNode::WhileNode { condition, 
                body } => {

            },
            ExpressionNode::ArrayDeclareNode { identifier, 
                elements_number, var_type } => {

            },
            ExpressionNode::ArrayAccessNode { identifier, 
                elements_number } => {

            },
            ExpressionNode::StructDeclareNode { identifier, 
                fields } => {

            },
            ExpressionNode::StructAccessNode { identifier, 
                field } => {

            },
            ExpressionNode::IfNode { condition, 
                body, _else } => {

            },
            ExpressionNode::ElseNode { _if, 
                body } => {
                
            },
            _ => panic!("wrong current node"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    None,
    ProgramNode {
        expressions : Vec<Rc<ExpressionNode>>,
    },
    ConstNode {
        number : TokenType, //number
    },
    DeclareVar {
        var_node : Rc<ExpressionNode>, //VarNode
        expression : Rc<ExpressionNode>, //assign operations
    },
    VarNode {
        var_type : TokenType,
        identifier : TokenType, //id
    },
    StatementNode {
        nodes : Vec<Rc<ExpressionNode>>,
    },
    BinaryOperation {
        operator : TokenType,
        left_operand : Rc<ExpressionNode>, //var node
        right_operand : Rc<ExpressionNode>,//var or function call or bin operation or array access or struct access
    },
    ConditionNode {
        nodes : Vec<Rc<ExpressionNode>>, //var nodes or bin operation
    },
    CallFunctionNode {
        identifier : TokenType,
        arguments : Vec<Rc<ExpressionNode>>, //var nodes or bin operation
    },
    DeclareFunctionNode {
        return_type : TokenType,
        identifier : TokenType,
        params : Vec<Rc<ExpressionNode>>, //var nodes
        body : Rc<ExpressionNode> //statement node
    },
    //Primary node for if/while
    WhileNode {
        condition : Rc<ExpressionNode>,
        body : Rc<ExpressionNode>,
    },
    IfNode {
        condition : Rc<ExpressionNode>,
        body : Rc<ExpressionNode>,
        _else : Rc<ExpressionNode>,
    },
    ElseNode {
        _if : Rc<ExpressionNode>,
        body : Rc<ExpressionNode>,
    },
    ArrayDeclareNode {
        var_type : TokenType,
        identifier : TokenType,
        elements_number : Rc<ExpressionNode>, //id or number or expression
    },
    ArrayAccessNode {
        identifier : TokenType,
        elements_number : Rc<ExpressionNode>, //variable node or binary operation
    },
    StructDeclareNode {
        identifier : TokenType,
        fields : Vec<ExpressionNode>, //variable nodes
    },
    StructAccessNode {
        identifier : TokenType,
        field : Rc<ExpressionNode>, //can be multiple struct access nodes
    },
}

impl ExpressionNode {
    pub fn new() -> Self {
        Self::None
    }

    pub fn new_const_node(number : TokenType) -> Self {
        ExpressionNode::ConstNode { 
            number
        }
    }

    pub fn new_id_node(id : TokenType, var_type : TokenType) -> Self {
        ExpressionNode::VarNode { 
            identifier : id, 
            var_type 
        }
    }

    pub fn new_statement_node() -> Self {
        ExpressionNode::StatementNode { 
            nodes : Vec::new(), 
        }
    }
}


pub struct Ast_tree {
    token_list : VecDeque<TokenType>,
    space_counter : u32,
    current_token : TokenType,
    previous_token : TokenType,
    current_id : u32,

    tmp_name : String,
    tmp_type : String,
}

impl Ast_tree {
    pub fn new(new_token_list : VecDeque<TokenType>) -> Self {
        Ast_tree { 
            token_list : new_token_list,
            space_counter : 0,

            current_token : TokenType::default(),
            previous_token : TokenType::default(),

            current_id : 0,

            tmp_name : String::new(),
            tmp_type : String::new(),
        }
    }

    pub fn write_out(&mut self) {
        println!("Program");
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
                    
                    self.space_counter += 1;
                    self.out_spaces();
                    self.struct_declare();
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
                    self.var();
                    self.next_token();
                    match self.current_token {
                        TokenType::Semicolon => {
                            continue;
                        }
                        _ => {}
                    }
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
                        self.space_counter+=1;
                        self.var();
                        self.space_counter-=1;
                    }
                    TokenType::OpenningParenthesis => {
                        self.out_spaces();
                        println!("FunctionDeclare");
                        self.space_counter+=1;
                        self.out_spaces();
                        println!("{}", self.tmp_type);
                        self.out_spaces();
                        println!("{}", self.tmp_name);
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
        self.out_spaces();
        println!("{}", self.tmp_type);
                
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

    }

    fn function_body(&mut self) {
        //println!("{:?}", self.current_token);
        self.next_token();
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
                _ => {} 
            }
        }
    }

    fn next_token(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.token_list.pop_front().unwrap();
        self.current_id+=1;
    }

    fn out_spaces(&mut self) {
        let mut i = 0;
        while i < self.space_counter {
            print!(" ");
            i+=1;
        }
    }
}