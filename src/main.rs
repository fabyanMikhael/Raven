#![allow(non_snake_case)]
use interpreter::interpreter::ParseFile;

pub mod interpreter;
mod tests;

fn main() {
    println!("{:?}",ParseFile("test.rv"));
}