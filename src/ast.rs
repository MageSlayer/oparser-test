
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Number(i32),
    VecInt(Vec<i32>)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AST<'a> {
    Value(Value),
    Verb(&'a ASTNode<'a>, &'a ASTNode<'a>),
    List(&'a ASTNode<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn is_monovec<'a>(n:&'a Vec<ASTNode<'a>>) -> bool {
    let isvec = n.iter().all( |x| match x {
        &ASTNode::Node(AST::Value(Value::Number(_))) => true,
        _ => false
    });
    isvec
}

fn to_monovec<'a,'b>(n:&'a Vec<ASTNode<'a>>) -> AST<'b> {
    let mut r:Vec<i32> = Vec::new();

    for v in n.iter() {
      match v {
         &ASTNode::Node(AST::Value(Value::Number(x))) => { r.push(x); },
         _ => panic!("Unexpected non-number")
      }
    }
    AST::Value(Value::VecInt(r))
}

pub fn postprocess<'a,'b>(n:&'b ASTNode<'a>, arena: &'a Arena<'a>) -> ASTNode<'a> {
    match n {
      &ASTNode::Node(ref x) => {
        ASTNode::Node(
          match x {
            &AST::Verb(ref a, ref b) => {
               AST::Verb( arena.alloc( postprocess(a, arena) ), arena.alloc( postprocess(b, arena) ) )
            },
            &AST::Value(ref x) => {
               AST::Value( x.clone() )
            },
            &AST::List(ref x) => {
               AST::List( arena.alloc( postprocess(x, arena) ) )
            }
          })
      },
      &ASTNode::Vec(ref x) => {
        if is_monovec(x) { 
            ASTNode::Node(to_monovec(x))
        } else { 
            let v = x.iter().map(|x| postprocess(x, arena) ).collect();
            ASTNode::Vec( v )
        }
      },
      x => x.clone()
    }
}
