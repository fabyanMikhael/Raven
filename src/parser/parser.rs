use ::std::fs::read_to_string;
use std::{rc::Rc, collections::HashMap, ops::Index, cell::RefCell, fmt::Debug};

use crate::interpreter::interpreter::{Scope, RefScope, Object, Interpreter, FunctionTypes};

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
    CreateFunction{name: Box<Type>, code: Vec<Box<Type>>, parameters: Vec<String>},
    Function(FunctionTypes)    
}

impl Into<Rc<RefCell<Self>>> for Type{
    fn into(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl Type{
    pub fn wrap(self) -> Object{
        Rc::new(RefCell::new(self))
    }
    pub fn toString(&self) -> String{
        match self {
            Type::Number(e) =>    format!("{}", e),
            Type::Symbol(e) => format!("{}", e),
            Type::String(e) => format!("{}", e),
            _ => format!("{:?}", self)
        }
    }
    pub fn Add(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x+*y).wrap()},
            (Type::String(x), Type::String(y)) => {Type::String(x.to_owned()+y).wrap()},
            (Type::String(x), Type::Number(y)) => {Type::String(x.to_owned()+&format!("{}", y)).wrap()},
            (Type::Number(x), Type::String(y)) => {Type::String(format!("{}", x) + y).wrap()},
            _ => panic!("attempted to Add {:?} with {:?}",x,y),
        }
    }
    pub fn Subtract(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x-*y).wrap()},
            _ => panic!("attempted to Subtract {:?} with {:?}",x,y),
        }
    }
    pub fn Multiply(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x * *y).wrap()},
            (Type::String(x), Type::Number(y)) => {Type::String(x.repeat(*y as usize)).wrap()},
            (Type::Number(x), Type::String(y)) => {Type::String(y.repeat(*x as usize)).wrap()},
            _ => panic!("attempted to Multiply {:?} with {:?}",x,y),
        }
    }
    pub fn Divide(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x / *y).wrap()},
            _ => panic!("attempted to Divide {:?} with {:?}",x,y),
        }
    }
}



peg::parser!{
    
    pub grammar RavenParser() for str {
        rule whitespace()
        = [' '| '\t' | '\n' | '\r' |'\u{A}']
        rule _ 
        = whitespace()*
        rule __
        = whitespace()+

        rule number() -> Type
        = n:$(['0'..='9' | '.' | '-']+) { Type::Number(n.parse::<f32>().unwrap_or_else(|_|panic!("value: {} is not a valid number!", n)))}
        
        rule Arithmetic() -> Type
        = precedence!{
            x:Atom() _ "+" _ y:Atom() { Type::Call{function: Box::new(Type::Symbol("__add__".to_owned())), arguments: vec![x,y] } }
            x:Atom() _ "-" _ y:Atom() { Type::Call{function: Box::new(Type::Symbol("__sub__".to_owned())), arguments: vec![x,y] } }
            --
            x:Atom() _ "*" _ y:Atom() { Type::Call{function: Box::new(Type::Symbol("__mult__".to_owned())), arguments: vec![x,y] } }
            x:Atom() _ "/" _ y:Atom() { Type::Call{function: Box::new(Type::Symbol("__div__".to_owned())), arguments: vec![x,y] } }
            --
            x:Atom() _ "^" _ y:Atom() { Type::Call{function: Box::new(Type::Symbol("__pow__".to_owned())), arguments: vec![x,y] } }
            --
            "(" e:Arithmetic() ")" { e }
        }

        rule symbol() -> Type
        = n:$(['A'..='z']+['0'..='9']*) { Type::Symbol(n.to_string()) }
        
        rule string() -> Type
        = "\"" n:$(['A'..='z' | '0'..='9' | ' ']*) "\"" { Type::String(n.to_string())}
    
        rule call() -> Type
        = _ sym:symbol() _ "(" expr:(parse() ** ",") ")"  &_  {Type::Call{function: Box::new(sym), arguments: expr}}

        rule chain_call() -> Type
        = _ "$" _ sym:symbol() _ expr:( parse() ** " ")  _  {Type::Call{function: Box::new(sym), arguments: expr}}

        rule pipe() -> Type
        = prev:Atom() _ "|>" _ expr:(pipe_call() ** "|>") {
            let mut last: Option<Type> = None;
            for func in expr {
                if let Type::Call {function, mut arguments } = func {
                    if let Some(ref last_func) = last {
                        arguments.insert(0, last_func.clone())
                    } else {
                        arguments.insert(0, prev.clone());
                    }

                    last = Some(Type::Call{function, arguments});
                }
            }

            return last.unwrap_or_else(|| panic!("attempted to pipe into a non-function object"))
        }

        rule function() -> Type
        = _ "fn" _ name:symbol()? _ "(" parameters:(symbol() ** ",") ")" _ "{" _ code:parseBlock() _ "}" _ {
            let name = name.unwrap_or_else(|| Type::Symbol("".to_owned()));
            let name = Box::new(name);
            let code = code.into_iter().map(Box::new).collect();
            let parameters = parameters.into_iter().map(|e| e.toString()).collect();
            Type::CreateFunction { name, code, parameters }
        }

        rule assignment() -> Type
        = _ name:symbol() _ "=" _ expr:parse() _ {
            Type::Assignment { variable: Box::new(name), value: Box::new(expr) }
        }
        rule declaration() -> Type
        = _ "let" _ name:symbol() _ "=" _ expr:parse() _ {
            Type::VariableDeclaration { variable: Box::new(name), value: Box::new(expr) }
        }

        rule Atom() -> Type = precedence!{
            n:chain_call() {n}
            --
            n:call()   {n}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
        }

        rule parse_intermediate() -> Type = precedence!{
            n:declaration() {n}
            --
            n:assignment() {n}
            --
            n:pipe() {n}
            --
            n:function() {n}
            n:chain_call() {n}
            --
            n:call()   {n}
            --
            n:Arithmetic() {n}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
        }

        rule parse() -> Type = 
        _ n:parse_intermediate() &_  {n}


        rule pipe_call() -> Type = precedence! {
            _ n:chain_call() _ {n}
            --
            _ n:call() _ {n}
        }
    
        rule parseBlock() -> Vec<Type> =
            _ code: ((x:parse() (";"/"\n"/_) {x})*) _ {code}

        pub rule ParseFile() -> Vec<Type> =
            code:parseBlock() {code}       
    }
}

pub fn ParseFile(file: &str) -> Vec<Type>{
    RavenParser::ParseFile(&read_to_string(file).unwrap()).unwrap()
}
pub fn ParseString(code: &str) -> Vec<Type>{
    RavenParser::ParseFile(code).unwrap()
}