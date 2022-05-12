#![allow(unused_imports)]
use crate::parser::parser::ParseString;
use crate::parser::parser::Type;

#[test]
pub fn basic(){
    const CODE: &str = "10";
    let ast = ParseString(CODE);
    assert_eq!(ast, vec![Type::Number(10.0)])
}

#[test]
pub fn function_call(){
    println!("beginning first test");
    const CODE: &str     = r##"print("hello")"##;
    let ast     = ParseString(CODE);
    let print   = Box::new(Type::Symbol("print".to_owned()));
    let args    = vec![Type::String("hello".to_owned())];
    let expected       = vec![Type::Call{function: print,arguments: args}];
    assert_eq!(ast, expected);

    println!("beginning second test");
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
    

    let arg_0   = vec![ten, hello, Type::Call{function: pop, arguments: vec![]}];
    let arg_1   = vec![Type::Call{function: anchor, arguments: arg_0}];
    let expected= vec![Type::Call{function: print, arguments: arg_1}];
    assert_eq!(ast, expected);
}

#[test]
pub fn functional(){

    println!("beginning first test");
    const BASIC: &str     = r##"$ test 10 "hello""##;
    let ast     = ParseString(BASIC);
    let print   = Box::new(Type::Symbol("test".to_owned()));
    let hello       = Type::String("hello".to_owned());
    let ten         = Type::Number(10.0);

    let expected       = vec![Type::Call{function: print, arguments: vec![ten, hello]}];

    assert_eq!(ast, expected);

    println!("beginning second test");
    const BASIC2: &str     = r##"$ test  10   "hello"   "##;
    let ast     = ParseString(BASIC2);
    assert_eq!(ast, expected);
}

#[test]
pub fn function(){
    const BASIC: &str     = r##"fn print(){

    }"##;

    let ast     = ParseString(BASIC);
    let print   = Box::new(Type::Symbol("print".to_owned()));
    let function         = Type::CreateFunction { name: print, code: Vec::new() };
    let expected       = vec![function];
    assert_eq!(ast, expected);
}


#[test]
pub fn assignment(){
    const BASIC: &str     = r##"y = 20;"##;
    let ast     = ParseString(BASIC);
    let variable   = Type::Symbol("y".to_owned()).into();
    let function         = Type::Assignment { variable, value: Type::Number(20.0).into() };
    let expected       = vec![function];
    assert_eq!(ast, expected);
}

#[test]
pub fn declaration(){
    const BASIC: &str     = r##"let y = 20;"##;
    let ast     = ParseString(BASIC);
    let variable   = Type::Symbol("y".to_owned()).into();
    let function         = Type::VariableDeclaration { variable, value: Type::Number(20.0).into() };
    let expected       = vec![function];
    assert_eq!(ast, expected);
}

#[test]
pub fn declaration_and_assignment(){
    const BASIC: &str     = r##"let y = 20;
    y = 10;
    let x = "hello";
    x = "hi";
    "##;
    let ast     = ParseString(BASIC);
    let y   = Type::Symbol("y".to_owned()).into();
    let x   = Type::Symbol("x".to_owned()).into();
    let mut expected       = vec![];

    expected.push(Type::VariableDeclaration { variable: y, value: Type::Number(20.0).into() });
    let y   = Type::Symbol("y".to_owned()).into();
    expected.push(Type::Assignment { variable: y, value: Type::Number(10.0).into() });

    expected.push(Type::VariableDeclaration { variable: x, value: Type::String("hello".into()).into() });
    let x   = Type::Symbol("x".to_owned()).into();
    expected.push(Type::Assignment { variable: x, value: Type::String("hi".into()).into() });
    
    assert_eq!(ast, expected);
}

// #[test]
// pub fn functional_chained(){

//     const BASIC: &str     = r##"$ test 10 "hello" $ pop 2.4 "##;

//     let ast     = ParseString(BASIC);
//     let print   = Box::new(Type::Symbol("test".to_owned()));
//     let hello       = Type::String("hello".to_owned());
//     let ten         = Type::Number(10.0);
    
//     let expected       = vec![Type::Call(print,vec![ten, hello])];

//     assert_eq!(ast, expected);
// }