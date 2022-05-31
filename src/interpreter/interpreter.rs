use std::{collections::HashMap, rc::Rc, cell::RefCell};
use std::fmt::Debug;
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Clone, PartialEq)]
pub enum FunctionTypes{
    NormalFunction{code: Vec<Box<Type>>, scope: RefScope, parameters: Vec<String>},
    BuiltIn{Function: Func, parameters: u8},
}
impl Debug for FunctionTypes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NormalFunction { code, scope, parameters } => f.debug_struct("NormalFunction").field("code", code).field("parameters", parameters).finish(),
            Self::BuiltIn { Function, parameters } => f.debug_struct("BuiltIn").field("Function", Function).field("parameters", parameters).finish(),
        }
    }
}

impl FunctionTypes{
    pub fn RunCode(code: &Vec<Box<Type>>, scope: RefScope) -> Option<Object>{
        let mut result = None;
        for line in code{
            result = Interpreter::interpret(*line.clone(), scope.clone());
        }
        result
    }
    pub fn isEnoughArgs(this: &Self, amount: u8) -> bool{
        match this{
            FunctionTypes::NormalFunction { code: _, scope: _, parameters } => amount as usize == parameters.len(),
            FunctionTypes::BuiltIn { Function: _, parameters } => amount >= *parameters,
        }
    }

    pub fn call(this: &Self, _function: Object, evaluated_arguments: Vec<Rc<RefCell<Type>>>, scope: RefScope) -> Option<Object>{

        match this {
            FunctionTypes::NormalFunction { code, scope, parameters } => {
                let new_scope = (**scope).borrow().clone();
                let new_scope = Rc::new(RefCell::new(new_scope));

                if evaluated_arguments.len() < parameters.len(){
                    let new_code = code.clone();
                    let mut new_parameters = parameters.clone();
                    for (arg,param) in evaluated_arguments.into_iter().zip(parameters){
                        new_scope.borrow_mut().declare(param.clone(), arg.clone());
                        new_parameters.remove(0);
                    }
                    let function = FunctionTypes::NormalFunction { code: new_code, scope: new_scope, parameters: new_parameters };
                    let functionObj = Type::Function(function);
                    Some(functionObj.wrap())
                }else{
                    for (parameter,argument) in parameters.iter().zip(evaluated_arguments){
                        new_scope.borrow_mut().declare(parameter.clone(), argument);
                    }
                    FunctionTypes::RunCode(&code, new_scope)
                }
            },
            FunctionTypes::BuiltIn { Function, parameters } => {
                if evaluated_arguments.len() < *parameters as usize{panic!("Called function with {} parameters. Expected {} or more*", evaluated_arguments.len(), parameters)} 
                return Function.0(scope.clone(), evaluated_arguments);
            },
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

    pub fn addObject(&mut self, name: &str, value: Type){
        self.global.borrow_mut().declare(name.to_string(), value.wrap());
    }

    pub fn run(&mut self, code: String, debug: bool){
        let node = ParseString(&code);
        // println!("{:#?}", node);

        if debug {
            print!("----------------\n");
            for n in &node {
                println!("{}\n", n.to_string(0, 0));
            }
            print!("----------------\n");
        }

        Self::interpretCode(node, self.global.clone());
    }

    fn interpretCode(code: Vec<Type>, scope: RefScope) -> Option<Object> {
        let mut result = None;
        for node in code{
            result = Self::interpret(node, scope.clone());
        }

        result
    }

    fn interpret(node: Type, scope: RefScope) -> Option<Object>{
        match node{
            Type::Call { function, arguments } => {
                let name = Self::Symbol(*function);
                let arguments = arguments.into_iter()
                .map(|e| Interpreter::interpret(e, scope.clone()).unwrap_or_else(|| panic!("cannot use void as argument")))
                .collect::<Vec<_>>();
                // println!("args {:?}", arguments);
                let functionObject = scope.borrow_mut().get(&name).0.clone();
                if let Type::Function(function) = &*(*functionObject).borrow(){
                    return FunctionTypes::call(function, functionObject.clone(), arguments, scope.clone())
                }
                None
            },
            Type::VariableDeclaration { variable, value } => {
                let result = Self::interpret(*value, scope.clone()).unwrap_or_else(|| panic!("attempted to assign void to a variable!"));
                scope.borrow_mut()
                .declare(Self::Symbol(*variable), result);
                None
            },
            Type::Assignment { variable, value } => {
                let new_value = Interpreter::interpret(*value, scope.clone()).unwrap();
                scope.borrow_mut()
                .assign(Self::Symbol(*variable), new_value);
                None 
            },
            Type::CreateFunction { name, code, parameters } => {
                let function  = FunctionTypes::NormalFunction { code, scope: scope.clone(), parameters };
                let function = Type::Function(function).wrap();
                scope.borrow_mut()
                .declare(Self::Symbol(*name), function.clone());
                Some(function)
            },
            Type::Conditional { condition, then, otherwise } => {
                if let Type::Bool(condition) = &*(*Interpreter::interpret(*condition, scope.clone()).unwrap()).borrow() {
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
            Type::While { condition, code } => {
                let mut result = None;
                loop {
                    if let Type::Bool(cond) = &*(*Interpreter::interpret(*condition.clone(), scope.clone()).unwrap()).borrow() {
                        if !*cond { break }
                        result = Interpreter::interpretCode(code.clone(), scope.clone());
                    }
                }
              
                result
            },
            Type::Invocation { code } => {
                Interpreter::interpretCode(code.clone(), scope.clone())
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