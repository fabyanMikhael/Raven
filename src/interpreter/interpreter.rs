use std::{collections::HashMap, rc::Rc, cell::RefCell};

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
        self.map.get(key).unwrap_or_else(||panic!("cannot find {}", key)).clone()
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

pub struct Interpreter{
    global: RefScope
}
impl Interpreter{
    pub fn new() -> Interpreter{
        Interpreter { global: Scope::new() }
    }

    pub fn addFunction<T: 'static +  Fn(RefScope,Vec<Object>) -> Option<Object>>(&mut self, name: &str, f: T){
        let obj = Type::BuiltIn(Func::new(Box::new(f)));
        self.global.borrow_mut().declare(name.to_string(), Rc::new(RefCell::new(obj)));
    }

    pub fn run(&mut self, code: String){
        let node = ParseString(&code);
        // println!("running {:?}", node);
        Self::interpretCode(node, self.global.clone());
    }

    fn interpretCode(code: Vec<Type>, scope: RefScope){
        // println!("{:?}", code);
        for node in code{
            Self::interpet(node, scope.clone());
        }
    }

    fn interpet(node: Type, scope: RefScope) -> Option<Object>{
        match node{
            Type::Call { function, arguments } => {
                let name = Self::Symbol(*function);
                let arguments = arguments.into_iter().map(|e| Self::interpet(e, scope.clone()).unwrap_or_else(|| panic!("expected a value for function call"))).collect();

                // println!("calling function {} with arguments <{:?}>", &name, &arguments);

                let function = scope.borrow_mut().get(&name).0.clone();
                if let Type::Function { code, scope} = &*function.borrow(){
                  let mut result = None;
                    let code = code.iter().map(|e|*e.clone()).collect::<Vec<_>>();
                    for line in code{
                        result = Self::interpet(line, scope.clone());
                    }
                    return result;
                }

                if let Type::BuiltIn(func)  = &*function.borrow(){
                    return func.0(scope.clone(), arguments);
                }
                None
            },
            Type::VariableDeclaration { variable, value } => {
                // println!("beginning to interpret node <{:?}>", &value);
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
            Type::CreateFunction { name, code } => {
                let function  = Type::Function { code, scope: scope.clone() };
                scope.borrow_mut()
                .declare(Self::Symbol(*name), Rc::new(RefCell::new(function)));
                None
            },
            Type::Symbol(name) => {
                Some(scope.borrow().get(&name).get().clone())
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
        if let Type::String(str) = &*node.borrow(){
            return str.to_owned()
        }
        panic!("node <{:?}> was not a string!", node)
    }
}