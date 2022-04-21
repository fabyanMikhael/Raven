#![allow(unused_imports)]
use crate::interpreter::interpreter::ParseString;
use crate::interpreter::interpreter::Type;

#[test]
pub fn basic(){
    const CODE: &str = "10";
    let ast = ParseString(CODE);
    assert_eq!(ast, vec![Type::Number(10.0)])
}

#[test]
pub fn function_call(){
    const CODE: &str     = r##"print("hello")"##;
    let ast     = ParseString(CODE);
    let print   = Box::new(Type::Symbol("print".to_owned()));
    let args    = vec![Type::String("hello".to_owned())];
    let expected       = vec![Type::Call(print,args)];
    assert_eq!(ast, expected);

    const CODE2: &str     = r##"print("hello");"##;
    let ast = ParseString(CODE2);
    assert_eq!(ast, expected);
}

#[test]
pub fn function_nested(){
    const CODE: &str     = r##"print(anchor(10,"hello", pop()))"##;
    let ast     = ParseString(CODE);

    let print   = Box::new(Type::Symbol("print".to_owned()));
    let anchor  = Box::new(Type::Symbol("anchor".to_owned()));
    let pop     = Box::new(Type::Symbol("pop".to_owned()));
    let hello       = Type::String("hello".to_owned());
    let ten         = Type::Number(10.0);
    

    let arg_0   = vec![ten, hello, Type::Call(pop, vec![])];
    let arg_1   = vec![Type::Call(anchor, arg_0)];
    let expected= vec![Type::Call(print,arg_1)];
    assert_eq!(ast, expected);
}

#[test]
pub fn functional(){

    const BASIC: &str     = r##"$ test 10 "hello" "##;
    let ast     = ParseString(BASIC);
    let print   = Box::new(Type::Symbol("test".to_owned()));
    let hello       = Type::String("hello".to_owned());
    let ten         = Type::Number(10.0);
    
    let expected       = vec![Type::Call(print,vec![ten, hello])];

    assert_eq!(ast, expected);
}

#[test]
pub fn functional_chained(){

    const BASIC: &str     = r##"$ test 10 "hello" $ pop 2.4 "##;

    let ast     = ParseString(BASIC);
    let print   = Box::new(Type::Symbol("test".to_owned()));
    let hello       = Type::String("hello".to_owned());
    let ten         = Type::Number(10.0);
    
    let expected       = vec![Type::Call(print,vec![ten, hello])];

    assert_eq!(ast, expected);
}