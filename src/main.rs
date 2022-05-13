#![allow(non_snake_case)]
use std::{fs::read_to_string, cell::RefCell, rc::Rc};

use interpreter::interpreter::Interpreter;
use parser::parser::{ParseFile, Type};

pub mod parser;
mod tests;
pub mod vm;
pub mod interpreter;

fn main() {

    let mut i = Interpreter::new();
    i.addFunction("print", 1, |_, args|{
        let text = args.into_iter().map(|e| e.borrow().toString()).collect::<Vec<_>>().join(" ");
        println!("{}", text);
        None
    });
    i.addFunction("number", 1, |_, args|{
        Some(Type::Number(args[0].borrow().toString().parse::<f32>().unwrap()).into())
    });
    i.addFunction("__add__", 1, |_, args|{
        Some(Type::Add(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__sub__", 1, |_, args|{
        Some(Type::Subtract(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__mult__", 1, |_, args|{
        Some(Type::Multiply(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__div__", 1, |_, args|{
        Some(Type::Divide(args[0].clone(), args[1].clone()))
    });
    i.run(read_to_string("src/tests/scripts/basics.rv").unwrap());

    // let r = RavenParser::ParseFile(r##"print("hello")"##).unwrap();
    // println!("{:?}", r);
}
