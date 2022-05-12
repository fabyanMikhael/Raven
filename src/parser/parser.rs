use ::std::fs::read_to_string;
use std::{rc::Rc, collections::HashMap, ops::Index, cell::RefCell, fmt::Debug};

use crate::interpreter::interpreter::{Scope, RefScope, Object};

#[derive(Clone)]
pub struct Func(pub &'static dyn Fn(RefScope,Vec<Object>) -> Option<Object>);
impl Debug for Func{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Func").field(&(&self.0 as *const _ as usize)).finish()
    }
}
impl Func{
    pub fn new(value: Box<dyn Fn(RefScope,Vec<Object>) -> Option<Object>>) -> Self{
        let val: &'static dyn Fn(RefScope,Vec<Object>) -> Option<Object> = Box::leak(value);
        Self(val)
    }
}


impl PartialEq for Func{
    fn eq(&self, other: &Self) -> bool {
        let my_ptr = &self.0 as *const _ as usize;
        let other_ptr = &other.0 as *const _ as usize;
        my_ptr == other_ptr
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    Number(f32),
    Symbol(String),
    String(String),
    Call{function: Box<Type>, arguments: Vec<Type>},
    VariableDeclaration{variable: Box<Type>, value: Box<Type>},
    Assignment{variable: Box<Type>, value: Box<Type>},
    CreateFunction{name: Box<Type>, code: Vec<Box<Type>>},
    Function{code: Vec<Box<Type>>, scope: RefScope},
    BuiltIn(Func)
}

impl Into<Rc<RefCell<Self>>> for Type{
    fn into(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl Type{
    pub fn toString(&self) -> String{
        match self {
            Type::Number(e) =>    format!("{}", e),
            Type::Symbol(e) => format!("{}", e),
            Type::String(e) => format!("{}", e),
            _ => format!("{:?}", self)
        }
    }
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
        = n:$(['A'..='z']+['0'..='9']*) { Type::Symbol(n.to_string()) }
        
        rule string() -> Type
        = "\"" n:$(['A'..='z' | '0'..='9' | ' ']*) "\"" { Type::String(n.to_string())}
        
        rule call() -> Type
        = _ sym:symbol() _ "(" expr:(parse_ws() ** ",") ")" _ [';' | _]? _  {Type::Call{function: Box::new(sym), arguments: expr}}

        rule chain_call() -> Type
        = _ "$" _ sym:symbol() __ expr:( parse_forward() ** " ") _ [';' | _]? _  {Type::Call{function: Box::new(sym), arguments: expr}}

        //ignores whitespace behind
        rule parse_forward() -> Type
        = _ exp:parse() {exp}

        //ignores whitespace around
        rule parse_ws() -> Type
        = _ exp:parse() _ {exp}
        

        rule function() -> Type
        = _ "fn" _ name:symbol()? _ "(" _ ")" _ "{" _ code:(parse() ** ";") _ "}" _ {
            let name = name.unwrap_or_else(|| Type::Symbol("".to_owned()));
            let name = Box::new(name);
            let code = code.into_iter().map(Box::new).collect();
            Type::CreateFunction { name, code }
        }

        rule assignment() -> Type
        = _ name:symbol() _ "=" _ expr:parse() _ ";" _ {
            Type::Assignment { variable: Box::new(name), value: Box::new(expr) }
        }
        rule declaration() -> Type
        = _ "let" _ name:symbol() _ "=" _ expr:parse() _ ";" _ {
            Type::VariableDeclaration { variable: Box::new(name), value: Box::new(expr) }
        }

        rule parse() -> Type = precedence!{
            n:declaration() {n}
            --
            n:assignment() {n}
            --
            n:function() {n}
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