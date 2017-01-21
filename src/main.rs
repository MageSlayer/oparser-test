#![feature(test)]
extern crate test;
extern crate lalrpop_util;

mod ast;
mod parser;

use ast::*;

#[test]
fn test1() {
    let arena = Arena::new();
    let ast = parser::parse_Expression(&arena, &"(1+1;1;(1;1;1))".to_string()).unwrap();
    
    assert_eq!( ast,
        ASTNode::Node(AST::List(
            &ASTNode::Vec(vec![
                ASTNode::Node( AST::Verb( 
                                 &ASTNode::Node(AST::Value(Value::Number(1))),
                                 &ASTNode::Node(AST::Value(Value::Number(1))) )),
                ASTNode::Node(AST::Value(Value::Number(1))),
                ASTNode::Node( AST::List( &ASTNode::Vec(vec![ 
                                              ASTNode::Node(AST::Value(Value::Number(1))),
                                              ASTNode::Node(AST::Value(Value::Number(1))),
                                              ASTNode::Node(AST::Value(Value::Number(1)))
                                              ] ) ))
                ]))
        ));
}

#[test]
fn test2() {
    let arena = Arena::new();
    let ast = parser::parse_Expression(&arena, &"(1+1;1;(1;1;1))".to_string()).unwrap();
    let p_ast = postprocess(&ast, &arena);
    
    assert_eq!( p_ast,
        ASTNode::Node(AST::List(
            &ASTNode::Vec(vec![
                ASTNode::Node( AST::Verb( 
                                 &ASTNode::Node(AST::Value(Value::Number(1))),
                                 &ASTNode::Node(AST::Value(Value::Number(1))) )),
                ASTNode::Node( AST::Value(Value::Number(1))),
                ASTNode::Node( AST::List( &ASTNode::Node( AST::Value(Value::VecInt( vec![1,1,1])) )))
                ]))
        ));
}
