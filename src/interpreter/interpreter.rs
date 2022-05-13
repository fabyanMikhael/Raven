use std::{collections::HashMap, rc::Rc, cell::RefCell, borrow::Borrow};

use crate::parser::parser::{Type, ParseString, Func};

pub type Object = Rc<RefCell<Type>>;
pub type RefScope = Rc<RefCell<Scope>>;

#[derive(Debug, PartialEq)]
pub struct Slot(Object);
impl  Slot {
    pub fn new(object: Object) -> Rc<Slot>{
        Rc::new(Slot(object))
    }
    pub fn set(&mut self, new: Object){
        self.0 = new;
    }
    pub fn get(&self) -> Object{
        self.0.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct Scope{
    map: HashMap<String, Rc<Slot>>,
    parent: Option<RefScope>
}
impl Scope{
    pub fn new() -> RefScope{
        Rc::new(RefCell::new(Scope { map: HashMap::new(), parent: None }))
    }
    pub fn with(parent: RefScope) -> RefScope{
        Rc::new(RefCell::new(Scope { map: HashMap::new(), parent: Some(parent) }))
    }

    pub fn get(&self, key: &str) -> Rc<Slot>{
        match self.map.get(key){
            Some(val) => val.clone(),
            None => {
                if let Some(parent) = &self.parent{
                    (**parent).borrow().get(key)
                }else{
                    panic!("cannot find {}", key)
                }
            },
        }

    }
    pub fn declare(&mut self, key: String, value: Object){
        self.map.insert(key, Rc::new(Slot(value)));
    }
    pub fn assign(&mut self, key: String, value: Object){
        if let Some(slot) = self.map.get_mut(&key){
            Rc::get_mut(slot).unwrap().set(value)
        }else{
            if let Some(parent) = &mut self.parent{
                parent.borrow_mut().assign(key, value)
            }else{
                panic!("cannot assign variable <{}> because it does not exist", &key)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionTypes{
    NormalFunction{code: Vec<Box<Type>>, scope: RefScope, parameters: Vec<String>},
    BuiltIn{Function: Func, parameters: u8},
    // PartialFunction{function: Object, applied: Vec<Object>}
}
impl FunctionTypes{
    pub fn RunCode(code: &Vec<Box<Type>>, scope: RefScope) -> Option<Object>{
        let mut result = None;
        for line in code{
            result = Interpreter::interpet(*line.clone(), scope.clone());
        }
        result
    }
    pub fn isEnoughArgs(this: &Self, amount: u8) -> bool{
        match this{
            FunctionTypes::NormalFunction { code, scope, parameters } => amount as usize == parameters.len(),
            FunctionTypes::BuiltIn { Function, parameters } => amount >= *parameters,
            // FunctionTypes::PartialFunction { function, applied } => {
            //     if let Type::Function(func) = &*(**function).borrow(){
            //         FunctionTypes::isEnoughArgs(func, amount)
            //     }else{
            //         panic!("idk man")
            //     }
            // },
        }
    }

    pub fn call(this: &Self, function: Object, evaluated_arguments: Vec<Rc<RefCell<Type>>>, scope: RefScope) -> Option<Object>{

        match this {
            FunctionTypes::NormalFunction { code, scope, parameters } => {
                if evaluated_arguments.len() < parameters.len(){
                    // let partial = Rc::new(RefCell::new(Type::Function(FunctionTypes::PartialFunction { function, applied: evaluated_arguments })));
                    // Some(partial)
                    panic!("Called function with {} arguments. Expected {}", evaluated_arguments.len(), parameters.len());
                }else{
                    for (parameter,argument) in parameters.iter().zip(evaluated_arguments){
                        scope.borrow_mut().declare(parameter.clone(), argument);
                    }
                    FunctionTypes::RunCode(&code, scope.clone())
                }
            },
            FunctionTypes::BuiltIn { Function, parameters } => {
                if evaluated_arguments.len() < *parameters as usize{panic!("Called function with {} parameters. Expected {} or more*", evaluated_arguments.len(), parameters)} 
                return Function.0(scope.clone(), evaluated_arguments);
            },
            // FunctionTypes::PartialFunction { function, applied } => {
            //     let mut new_args = vec![];
            //     for i in applied{new_args.push(i.clone())};
            //     for i in evaluated_arguments{new_args.push(i)};
                
            //     println!("partial==");
            //     if let Type::Function(func) = &*(**function).borrow(){
            //         if FunctionTypes::isEnoughArgs(func, new_args.len() as u8){
            //             FunctionTypes::call(func, function.clone(), new_args, scope)
            //         }else{
            //             println!("partial");
            //             let partial = FunctionTypes::PartialFunction { function: function.clone(), applied: new_args};
            //             let partial = Type::Function(partial);
            //             let partial = Rc::new(RefCell::new(partial)); 
            //             Some(partial)
            //         }
            //     }else{
            //         None
            //     }
            // },
        }
    }
}



pub struct Interpreter{
    global: RefScope
}
impl Interpreter{
    pub fn new() -> Interpreter{
        Interpreter { global: Scope::new() }
    }

    pub fn addFunction<T: 'static +  Fn(RefScope,Vec<Object>) -> Option<Object>>(&mut self, name: &str, parameters: u8, f: T){
        let obj = FunctionTypes::BuiltIn {Function: Func::new(Box::new(f)), parameters};
        let obj = Type::Function(obj);
        self.global.borrow_mut().declare(name.to_string(), Rc::new(RefCell::new(obj)));
    }

    pub fn run(&mut self, code: String){
        let node = ParseString(&code);
        println!("{:#?}", node);
        Self::interpretCode(node, self.global.clone());
    }

    fn interpretCode(code: Vec<Type>, scope: RefScope) -> Option<Object> {
        let mut result = None;
        for node in code{
            result = Self::interpet(node, scope.clone());
        }

        result
    }

    fn interpet(node: Type, scope: RefScope) -> Option<Object>{
        // println!("interpreting: {:?}", node);
        match node{
            Type::Call { function, arguments } => {
                let name = Self::Symbol(*function);
                let arguments = arguments.into_iter()
                .map(|e| Interpreter::interpet(e, scope.clone()).unwrap_or_else(|| panic!("cannot use void as argument")))
                .collect::<Vec<_>>();
                // println!("args {:?}", arguments);
                let functionObject = scope.borrow_mut().get(&name).0.clone();
                if let Type::Function(function) = &*(*functionObject).borrow(){
                    return FunctionTypes::call(function, functionObject.clone(), arguments, scope.clone())
                }
                None
            },
            Type::VariableDeclaration { variable, value } => {
                let result = Self::interpet(*value, scope.clone()).unwrap_or_else(|| panic!("attempted to assign void to a variable!"));
                scope.borrow_mut()
                .declare(Self::Symbol(*variable), result);
                None
            },
            Type::Assignment { variable, value } => {
                scope.borrow_mut()
                .assign(Self::Symbol(*variable), Rc::new(RefCell::new(*value)));
                None 
            },
            Type::CreateFunction { name, code, parameters } => {
                let function  = FunctionTypes::NormalFunction { code, scope: scope.clone(), parameters };
                let function = Type::Function(function);
                scope.borrow_mut()
                .declare(Self::Symbol(*name), Rc::new(RefCell::new(function)));
                None
            },
            Type::Conditional { condition, then, otherwise } => {
                if let Type::Bool(condition) = &*(*Interpreter::interpet(*condition, scope.clone()).unwrap()).borrow() {
                    return if *condition {
                        Interpreter::interpretCode(then, scope.clone())
                    } else if let Some(otherwise) = otherwise {
                        Interpreter::interpretCode(otherwise, scope.clone())
                    } else {
                        None
                    }
                }
                None
            },
            Type::Symbol(name) => {
                let result = Some((*scope).borrow().get(&name).get().clone());
                result
                
            },
            node @ _ => Some(Rc::new(RefCell::new(node)))
        }
    }
    
    fn Symbol(node: Type) -> String{
        if let Type::Symbol(symbol) = node{
            return symbol
        }
        panic!("node <{:?}> was not a symbol!", node)
    }
    pub fn String(node: Object) -> String{
        if let Type::String(str) = &*(*node).borrow(){
            return str.to_owned()
        }
        panic!("node <{:?}> was not a string!", node)
    }
}