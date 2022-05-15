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
    i.addFunction("__pow__", 1, |_, args|{
        Some(Type::Power(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__equals__", 1, |_, args|{
        Some(Type::Equals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__not_equals__", 1, |_, args|{
        Some(Type::NotEquals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__leq__", 1, |_, args|{
        Some(Type::LessThanOrEquals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__gte__", 1, |_, args|{
        Some(Type::GreaterThanOrEquals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__lt__", 1, |_, args|{
        Some(Type::LessThan(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__gt__", 1, |_, args|{
        Some(Type::GreaterThan(args[0].clone(), args[1].clone()))
    });

    i.addObject("true", Type::Bool(true));
    i.addObject("false", Type::Bool(false));

    // i.addFunction("__if__", 1, |_, args|{
    //     Some(Type::NotEquals(args[0].clone(), args[1].clone()))
    // });

    // i.addFunction("__if__", 1, |_, args|{
    //     let n = args.len();

    //     if args[0].borrow() {

    //     }


    //     None
    // });



    i.run(read_to_string("src/tests/scripts/basics.rv").unwrap());






    // let r = RavenParser::ParseFile(r##"print("hello")"##).unwrap();
    // println!("{:?}", r);
}
