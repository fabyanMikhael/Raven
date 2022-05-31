#![allow(non_snake_case)]
use std::{fs::read_to_string, cell::RefCell, rc::Rc};

use interpreter::interpreter::Interpreter;
use parser::parser::{ParseFile, Type};

pub mod parser;
mod tests;
pub mod vm;
pub mod interpreter;

// #[cfg(target_arch = "wasm32")]
pub mod wasm;


// #[macro_use]
// extern crate lazy_static;



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
    i.addFunction("__mul__", 1, |_, args|{
        Some(Type::Multiply(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__div__", 1, |_, args|{
        Some(Type::Divide(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__pow__", 1, |_, args|{
        Some(Type::Power(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__mod__", 1, |_, args|{
        Some(Type::Modulo(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__eq__", 1, |_, args|{
        Some(Type::Equals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__ne__", 1, |_, args|{
        Some(Type::NotEquals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__le__", 1, |_, args|{
        Some(Type::LessThanOrEquals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__ge__", 1, |_, args|{
        Some(Type::GreaterThanOrEquals(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__lt__", 1, |_, args|{
        Some(Type::LessThan(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__gt__", 1, |_, args|{
        Some(Type::GreaterThan(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__and__", 1, |_, args|{
        Some(Type::And(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__or__", 1, |_, args|{
        Some(Type::Or(args[0].clone(), args[1].clone()))
    });
    i.addFunction("__not__", 1, |_, args|{
        Some(Type::Not(args[0].clone()))
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



    i.run(read_to_string("src/tests/scripts/basics.rv").unwrap(), true);






    // let r = RavenParser::ParseFile(r##"print("hello")"##).unwrap();
    // println!("{:?}", r);
}
