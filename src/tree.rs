use std::ops::DerefMut;

use crate::token::*;

#[derive(Debug, Clone)]
pub struct Tree {
    root : ExpressionNode,
    previous_nodes : Vec<ExpressionNode>,
    current_node : Box<ExpressionNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: ExpressionNode::ProgramNode { expressions: Vec::new() },
            current_node: Box::new(ExpressionNode::new()),
            previous_nodes : Vec::new(),
        }
    }

    pub fn add_node_to_current_node(&mut self, expression_node : &ExpressionNode) {
        match self.current_node.deref_mut() {
            ExpressionNode::ProgramNode { expressions } => {
                expressions.push(Box::new(expression_node.clone()));
            },
            ExpressionNode::StatementNode { nodes } => {
                nodes.push(Box::new(expression_node.clone()));
            },
            ExpressionNode::BinaryOperation { right_operand, .. } => {
                *right_operand = Box::new(expression_node.clone());
            },
            ExpressionNode::ConditionNode { nodes } => {
                nodes.push(Box::new(expression_node.clone()));
            },
            ExpressionNode::CallFunctionNode { arguments , .. } => {
                    arguments.push(Box::new(expression_node.clone()));
            },
            ExpressionNode::DeclareFunctionNode { identifier, 
                params, body } => {

            },
            ExpressionNode::WhileNode { condition, 
                body } => {

            },
            ExpressionNode::ArrayDeclareNode { identifier, 
                elements_number } => {

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

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    None,
    ProgramNode {
        expressions : Vec<Box<ExpressionNode>>,
    },
    ConstNode {
        number : TokenType, //number
    },
    VarNode {
        identifier : TokenType, //id
        var_type : TokenType,
    },
    StatementNode {
        nodes : Vec<Box<ExpressionNode>>,
    },
    BinaryOperation {
        operator : TokenType,
        left_operand : Box<ExpressionNode>, //var node
        right_operand : Box<ExpressionNode>,//var or function call or bin operation or array access or struct access
    },
    ConditionNode {
        nodes : Vec<Box<ExpressionNode>>, //var nodes or bin operation
    },
    CallFunctionNode {
        identifier : TokenType,
        arguments : Vec<Box<ExpressionNode>>, //var nodes or bin operation
    },
    DeclareFunctionNode {
        identifier : TokenType,
        params : Vec<Box<ExpressionNode>>, //var nodes
        body : Box<ExpressionNode> //statement node
    },
    //Primary node for if/while
    WhileNode {
        condition : Box<ExpressionNode>,
        body : Box<ExpressionNode>,
    },
    IfNode {
        condition : Box<ExpressionNode>,
        body : Box<ExpressionNode>,
        _else : Box<ExpressionNode>,
    },
    ElseNode {
        _if : Box<ExpressionNode>,
        body : Box<ExpressionNode>,
    },
    ArrayDeclareNode {
        identifier : TokenType,
        elements_number : Box<ExpressionNode>, //id or number or expression
    },
    ArrayAccessNode {
        identifier : TokenType,
        elements_number : Box<ExpressionNode>, //variable node or binary operation
    },
    StructDeclareNode {
        identifier : TokenType,
        fields : Vec<ExpressionNode>, //variable nodes
    },
    StructAccessNode {
        identifier : TokenType,
        field : Box<ExpressionNode>, //can be multiple struct access nodes
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