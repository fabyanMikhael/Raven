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
    i.addFunction("print", |_, args|{
        let text = args.into_iter().map(|e| e.borrow().toString()).collect::<Vec<_>>().join(" ");
        println!("{}", text);
        None
    });
    i.addFunction("number", |_, args|{
        Some(Type::Number(Interpreter::String(args[0].clone()).parse::<f32>().unwrap()).into())
    });
    i.run(read_to_string("src/tests/scripts/basics.rv").unwrap());
}