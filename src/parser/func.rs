use crate::interpreter::interpreter::Object;

use super::parser::Type;

impl Type{
    pub fn Add(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x+*y).wrap()},
            (Type::String(x), Type::String(y)) => {Type::String(x.to_owned()+y).wrap()},
            (Type::String(x), Type::Number(y)) => {Type::String(x.to_owned()+&format!("{}", y)).wrap()},
            (Type::Number(x), Type::String(y)) => {Type::String(format!("{}", x) + y).wrap()},
            _ => panic!("attempted to Add {:?} with {:?}",x,y),
        }
    }
    pub fn Subtract(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x-*y).wrap()},
            _ => panic!("attempted to Subtract {:?} with {:?}",x,y),
        }
    }
    pub fn Multiply(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x * *y).wrap()},
            (Type::String(x), Type::Number(y)) => {Type::String(x.repeat(*y as usize)).wrap()},
            (Type::Number(x), Type::String(y)) => {Type::String(y.repeat(*x as usize)).wrap()},
            _ => panic!("attempted to Multiply {:?} with {:?}",x,y),
        }
    }
    pub fn Divide(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x / *y).wrap()},
            _ => panic!("attempted to Divide {:?} with {:?}",x,y),
        }
    }

    pub fn Power(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number((*x).powf(*y)).wrap()},
            _ => panic!("attempted to Divide {:?} with {:?}",x,y),
        }
    }

    pub fn Modulo(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Number(*x % *y).wrap()},
            _ => panic!("attempted to Modulo {:?} with {:?}",x,y),
        }
    }

    pub fn Equals(x: Object, y: Object) -> Object{
        return Type::Bool(*x.borrow() == *y.borrow()).wrap()
    }

    pub fn NotEquals(x: Object, y: Object) -> Object{
        return Type::Bool(*x.borrow() != *y.borrow()).wrap()
    }

    pub fn LessThanOrEquals(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Bool(*x <= *y).wrap()},
            _ => panic!("attempted to compare {:?} with {:?}",x,y),
        }
    }

    pub fn GreaterThanOrEquals(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Bool(*x >= *y).wrap()},
            _ => panic!("attempted to compare {:?} with {:?}",x,y),
        }
    }

    pub fn LessThan(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Bool(*x < *y).wrap()},
            _ => panic!("attempted to compare {:?} with {:?}",x,y),
        }
    }

    pub fn GreaterThan(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Number(x), Type::Number(y)) => {Type::Bool(*x > *y).wrap()},
            _ => panic!("attempted to compare {:?} with {:?}",x,y),
        }
    }

    pub fn And(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Bool(x), Type::Bool(y)) => {Type::Bool(*x && *y).wrap()},
            _ => panic!("attempted to And {:?} with {:?}",x,y),
        }
    }

    pub fn Or(x: Object, y: Object) -> Object{
        match (&*x.borrow(),&*y.borrow()){
            (Type::Bool(x), Type::Bool(y)) => {Type::Bool(*x || *y).wrap()},
            _ => panic!("attempted to Or {:?} with {:?}",x,y),
        }
    }

    pub fn Not(x: Object) -> Object{
        match &*x.borrow(){
            Type::Bool(x) => {Type::Bool(!*x).wrap()},
            _ => panic!("attempted to Not {:?}",x),
        }
    }


}