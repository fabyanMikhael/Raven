#![allow(non_snake_case)]
use std::{fs::read_to_string, cell::RefCell, rc::Rc};

use interpreter::interpreter::Interpreter;
use parser::parser::{ParseFile, Type};

pub mod parser;
mod tests;
pub mod vm;
pub mod interpreter;

fn main() {

    // let mut i = Interpreter::new();
    // i.addFunction("print", |_, args|{
    //     let text = args.into_iter().map(|e| e.borrow().toString()).collect::<Vec<_>>().join(" ");
    //     println!("{}", text);
    //     None
    // });
    // i.addFunction("number", |_, args|{
    //     Some(Type::Number(Interpreter::String(args[0].clone()).parse::<f32>().unwrap()).into())
    // });
    // i.run(read_to_string("src/tests/scripts/basics.rv").unwrap());

    let r = RavenParser::ParseFile(r##"print("hello")"##).unwrap();
    println!("{:?}", r);
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
        = _ sym:symbol() _ "(" expr:(parse() ** ",") ")" _  {Type::Call{function: Box::new(sym), arguments: expr}}



        rule parse_() -> Type = precedence!{
            n:call()   {n}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
        }
        rule parse() -> Type = 
            _ n:parse_() &_ {n}

        pub rule ParseFile() -> Vec<Type> = precedence!{
            code:((parse())* ) {
                code
            }
        }
    }
}