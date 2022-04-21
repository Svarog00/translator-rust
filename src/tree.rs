use crate::token::*;

#[derive(Debug, Clone)]
pub struct Tree {
    root : ExpressionNode,
    current_node : Box<ExpressionNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: ExpressionNode::ProgramNode { expressions: Vec::new() },
            current_node: Box::new(ExpressionNode::new()),
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
        identifier : TokenType, //number
    },
    VarNode {
        identifier : TokenType, //id
        var_type : TokenType,
    },
    StatementNode {
        nodes : Vec<Box<ExpressionNode>>,
    },
    AssignNode {
        operator : TokenType,
        left_operand : Box<ExpressionNode>, //var node
        right_operand : Box<ExpressionNode>, //var or function call or bin operation or array access or struct access
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
    //Primary node for if/for/while
    PrimaryNode {
        condition : Box<ExpressionNode>,
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
}