use std::{ops::{DerefMut, Deref}, rc::Rc};

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

