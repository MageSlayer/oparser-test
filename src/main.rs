#![feature(test)]
extern crate test;
extern crate lalrpop_util;

mod ast;
mod parser;

use ast::*;

#[test]
fn test1() {
    let arena = Arena::new();
    let code = parser::parse_Expression(&arena, &"(1+1;1)".to_string()).unwrap();
    
    assert_eq!( code,
        ASTNode::Node(AST::List(
            &ASTNode::Vec(vec![
                ASTNode::Node( AST::Verb( 
                                 &ASTNode::Node(AST::Number(1)),
                                 &ASTNode::Node(AST::Number(1)) )),
                ASTNode::Node(AST::Number(1))
                ]))
        ));
}
