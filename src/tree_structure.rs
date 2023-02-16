use std::{rc::Rc, cell::RefCell};

use crate::token::*;

//Rc = reference counter. Rc::Copy делает копию УКАЗАТЕЛЯ, а не предмета, на который он указывает.
//RefCell = ????
pub struct TreeNode {
    pub value : Option<TokenType>,
    pub children : Vec<Rc<RefCell<TreeNode>>>,
    pub parent : Option<Rc<RefCell<TreeNode>>>, 
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode { value: None, children: vec![], parent: None }
    }

    pub fn init_node_with_token(token : TokenType) -> TreeNode {
        TreeNode { value: Some(token), children: vec![], parent: None }
    }

    pub fn add_child(&mut self, new_node : Rc<RefCell<TreeNode>>){
        self.children.push(new_node);
    }
}