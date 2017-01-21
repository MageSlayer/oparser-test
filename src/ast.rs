
#[derive(Debug, PartialEq, Eq)]
pub enum AST<'a> {
    Number(i32),
    Verb(&'a ASTNode<'a>, &'a ASTNode<'a>),
    List(&'a ASTNode<'a>)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ASTNode<'a> {
    Nil,
    Node(AST<'a>),
    Vec(Vec<ASTNode<'a>>)
}

use std::cell::RefCell;

pub struct Arena<'a> {
    data: RefCell<Vec<Box<ASTNode<'a>>>>
}

impl<'a> Arena<'a> {
    pub fn new() -> Arena<'a> {
        Arena { data: RefCell::new(vec![]) }
    }

    pub fn alloc(&'a self, n: ASTNode<'a>) -> &'a ASTNode<'a> {
        let b = Box::new(n);
        let p: *const ASTNode<'a> = &*b;
        self.data.borrow_mut().push(b);
        unsafe { &*p }
    }
}