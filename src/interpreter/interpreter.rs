use ::std::fs::read_to_string;
use std::{rc::Rc, collections::HashMap, ops::Index};

#[derive(Debug, PartialEq)]
pub enum Type{
    Number(f32),
    Symbol(String),
    String(String),
    Call(Box<Type>, Vec<Type>),
    Assignment(Box<Type>, Box<Type>)
}

peg::parser!{
    pub grammar RavenParser() for str {
        rule _ 
        = [' '| '\t' | '\n' | '\r' |'\u{A}']*
        rule __
        = [' '| '\t' | '\n' | '\r' |'\u{A}']+

        rule number() -> Type
        = n:$(['0'..='9' | '.' | '-']+) { Type::Number(n.parse::<f32>().unwrap_or_else(|_|panic!("value: {} is not a valid number!", n)))}
        
        rule symbol() -> Type
        = n:$(['A'..='z' | '*']+['0'..='9']*) { Type::Symbol(n.to_string()) }
        
        rule string() -> Type
        = "\"" n:$(['A'..='z' | ' ']*) "\"" { Type::String(n.to_string())}
        
        rule call() -> Type
        = _ sym:symbol() "(" _ expr:(parse() ** ",") _ ")" _ ";"? {Type::Call(Box::new(sym), expr)}

        rule chain_call() -> Type
        = _ "$" _ sym:symbol() __ expr:(parse() ** " ") _ ";"?  {Type::Call(Box::new(sym), expr)}

        rule parse() -> Type = precedence!{
            n:chain_call() {n}
            --
            n:call()   {n}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
        }
    
        pub rule ParseFile() -> Vec<Type> = precedence!{
            code:((parse())* ) {
                code
            }
        }
    }
}

pub fn ParseFile(file: &str) -> Vec<Type>{
    RavenParser::ParseFile(&read_to_string(file).unwrap()).unwrap()
}
pub fn ParseString(code: &str) -> Vec<Type>{
    RavenParser::ParseFile(code).unwrap()
}