use ::std::fs::read_to_string;
use std::{rc::Rc, cell::RefCell, fmt::Debug};

use crate::interpreter::interpreter::{RefScope, Object, FunctionTypes};

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

#[inline(always)]
fn bbox<T>(val: T) -> Box<T>{
    Box::new(val)
}

#[inline(always)]
fn sym(name: &str) -> Type{
    Type::Symbol(name.to_owned())
}

#[inline(always)]
fn bsym(name: &str) -> Box<Type> {
    Box::new(Type::Symbol(name.to_owned()))
}



#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    Number(f32),
    Bool(bool),
    Symbol(String),
    String(String),
    Call{function: Box<Type>, arguments: Vec<Type>},
    VariableDeclaration{variable: Box<Type>, value: Box<Type>},
    Assignment{variable: Box<Type>, value: Box<Type>},
    CreateFunction{name: Box<Type>, code: Vec<Box<Type>>, parameters: Vec<String>},
    Function(FunctionTypes),
    Conditional{condition: Box<Type>, then: Vec<Type>, otherwise: Option<Vec<Type>>},
    While{condition: Box<Type>, code: Vec<Type>},
    Invocation{code: Vec<Type>},
    Comment(String)
}

impl Into<Rc<RefCell<Self>>> for Type{
    fn into(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl Type {
    pub fn wrap(self) -> Object{
        Rc::new(RefCell::new(self))
    }
    pub fn toString(&self) -> String{
        match self {
            Type::Number(e) =>    format!("{}", e),
            Type::Symbol(e) => format!("{}", e),
            Type::String(e) => format!("{}", e),
            Type::Bool(e) => format!("{}", e),
            _ => format!("{:?}", self)
        }
    }

    #[inline(always)]
    pub fn c(&self) -> Type{
        self.clone()
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
            x:(@) _ "+" _  y:@ { Type::Call{function: bsym("__add__"), arguments: vec![x,y] } }
            x:(@) _ "-" _  y:@ { Type::Call{function: bsym("__sub__"), arguments: vec![x,y] } }
            --
            x:(@) _ "*" _  y:@ { Type::Call{function: bsym("__mul__"), arguments: vec![x,y] } }
            x:(@) _ "/" _  y:@ { Type::Call{function: bsym("__div__"), arguments: vec![x,y] } }
            --
            x:(@) _ "**" _  y:@ { Type::Call{function: bsym("__pow__"), arguments: vec![x,y] } }
            x:(@) _ "%" _  y:@ { Type::Call{function: bsym("__mod__"), arguments: vec![x,y] } }
            --
            x:Atom() {x}
            "(" _ x:Arithmetic() _ ")" _ {x}
        }


        rule Operation() -> Type
        = precedence!{
            x:symbol() _ "++" _ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__add__"), arguments: vec![x, Type::Number(1.0)] }) } }
            x:symbol() _ "--" _ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__sub__"), arguments: vec![x, Type::Number(1.0)] }) } }
            x:symbol() _ "+=" _  y:@ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__add__"), arguments: vec![x,y] }) } }
            x:symbol() _ "-=" _  y:@ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__sub__"), arguments: vec![x,y] }) } }
            x:symbol() _ "*=" _  y:@ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__mul__"), arguments: vec![x,y] }) } }
            x:symbol() _ "/=" _  y:@ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__div__"), arguments: vec![x,y] }) } }
            x:symbol() _ "**=" _  y:@ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__pow__"), arguments: vec![x,y] }) } }
            x:symbol() _ "%=" _  y:@ { Type::Assignment { variable: bbox(x.c()), value: bbox(Type::Call{function: bsym("__mod__"), arguments: vec![x,y] }) } }
            --
            x:(@) _ "&&" _ y:@ { Type::Call{function: bsym("__and__"), arguments: vec![x,y] } }
            x:(@) _ "||" _ y:@ { Type::Call{function: bsym("__or__"), arguments: vec![x,y] } }
            --
            x:(@) _ "==" _ y:@ { Type::Call{function: bsym("__eq__"), arguments: vec![x,y] } }
            x:(@) _ "!=" _ y:@ { Type::Call{function: bsym("__ne__"), arguments: vec![x,y] } }
            --
            x:(@) _ "<=" _ y:@ { Type::Call{function: bsym("__le__"), arguments: vec![x,y] } }
            x:(@) _ ">=" _ y:@ { Type::Call{function: bsym("__ge__"), arguments: vec![x,y] } }
            --
            x:(@) _ "<" _ y:@ { Type::Call{function: bsym("__lt__"), arguments: vec![x,y] } }
            x:(@) _ ">" _ y:@ { Type::Call{function: bsym("__gt__"), arguments: vec![x,y] } }
            --
            "!" _ x:(@) { Type::Call{function: bsym("__not__"), arguments: vec![x] } }
            --
            x:Arithmetic() {x}
            "(" _ x:Operation() _ ")" _ {x}       
        }


        rule symbol() -> Type
        = n:$(['A'..='z'](['0'..='9'] / ['A'..='z'])*) { Type::Symbol(n.to_string()) }
        
        rule spaced_symbol() -> Type
        = _ n:symbol() _ {n}

        rule string() -> Type
        = "\"" n:$([^ '"']*) "\"" { Type::String(n.to_string())}
    
        rule call() -> Type
        = _ sym:symbol() _ "(" expr:(parse() ** ",") ")"  &_  {Type::Call{function: bbox(sym), arguments: expr}}

        rule chain_call() -> Type
        = _ "$" _ sym:symbol() _ expr:(Operation() ** " ")  _  {Type::Call{function: bbox(sym), arguments: expr}}

        rule comment() -> Type
        = _ "//" n:$([^ '\n']*) "\n"? {Type::Comment(n.to_string())}

        rule pipe_right() -> Type
        = _ start:(start:Operation() _ "|>" _ {start})? _ expr:(pipe_call_right() ++ "|>") _  {
            let mut last = start;
            for func in expr {
                if let Type::Call { function, mut arguments } = func {
                    if let Some(last_func) = last {
                        arguments.insert(0, last_func);
                    }
                    last = Some(Type::Call { function, arguments});                   
                }
            }
            return last.unwrap()
        }

        rule pipe_left() -> Type
        = _ expr:(pipe_call_left() ++ "<|") _ end:("<|" _ end:Operation() _ {end})? _ {
            let mut last = end;
            for func in expr.into_iter().rev() {
                if let Type::Call { function, mut arguments } = func {
                    if let Some(last_func) = last {
                        arguments.push(last_func);
                    }
                    last = Some(Type::Call { function, arguments});                   
                }
            }
            return last.unwrap()
        }

        rule Else() -> Vec<Type>
        = code:bracket_block() {code}
        rule Elif() -> Vec<Type>
        = code: if_condition() {vec![code]}
        rule else_elif() -> Vec<Type>
        = "else" _ res:(Else() / Elif()) {res}
        rule if_condition() -> Type
        = _ "if" _ "("? _ condition:Operation() _ ")"? _ then:bracket_block() _ otherwise:(else_elif())? _ {
            Type::Conditional{condition: bbox(condition), then, otherwise}
        }

        rule while_loop() -> Type
        = _ "while" _ "("? _ condition:Operation() _ ")"? _ code:bracket_block() _ {
            Type::While{condition: bbox(condition), code}
        }

        rule function() -> Type
        = _ "fn" _ name:symbol() _ "(" _ parameters:(spaced_symbol() ** ",") _ ")" _ code:bracket_block() _ {
            let name = bbox(name);
            let code = code.into_iter().map(Box::new).collect();
            let parameters = parameters.into_iter().map(|e| e.toString()).collect();
            Type::CreateFunction { name, code, parameters }
        }

        rule lambda() -> Type
        = _ "(" _ parameters:(spaced_symbol() ** ",") _ ")" _ "=>" _ code:bracket_block() _ {
            let parameters = parameters.into_iter().map(|e| e.toString()).collect();
            let code = code.into_iter().map(Box::new).collect();
            Type::CreateFunction { name: bsym(""), parameters, code }
        }

        rule assignment() -> Type
        = _ name:symbol() _ "=" _ expr:parse() _ {
            Type::Assignment { variable: bbox(name), value: bbox(expr) }
        }
        rule declaration() -> Type
        = _ "let" _ name:symbol() _ "=" _ expr:parse() _ {
            Type::VariableDeclaration { variable: bbox(name), value: bbox(expr) }
        }


        rule Atom() -> Type = precedence!{
            n:chain_call() {n}
            --
            n:call()   {n}
            --
            n:while_loop() {n}
            n:if_condition() {n}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
            --
            _ "(" _ e:Atom() _ ")" _ { e }
        }

        rule parse_intermediate() -> Type = precedence!{
            n:comment() {n}
            --
            n:declaration() {n}
            --
            n:assignment() {n}
            --
            // n:dual_pipe() {n}
            // --
            n:pipe_left() {n}
            n:pipe_right() {n}
            --
            n:lambda() {n}
            n:function() {n}
            n:chain_call() {n}
            --
            n:if_condition() {n}
            n:while_loop() {n}
            --
            // n:Arithmetic() {n}
            // --
            n:Operation() {n}
            --
            n:call()   {n}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
        }

        rule parse() -> Type = 
        _  n:parse_intermediate() &_  {n}
        
        rule pipe_call_right() -> Type = precedence! {
            _ n:pipe_left() _ {n}
            --
            _ n:chain_call() _ {n}
            --
            _ n:call() _ {n}
        }

        rule pipe_call_left() -> Type = precedence! {
            // _ n:pipe_right() _ {n}
            // --
            _ n:chain_call() _ {n}
            --
            _ n:call() _ {n}
        }

        rule bracket_block() -> Vec<Type>
        = "{" _ code:parseBlock() _ "}" {code}
    
        rule parseBlock() -> Vec<Type> =
            _ code:((x:parse() (";"/"\n"/_) {x})*) _ {code}

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