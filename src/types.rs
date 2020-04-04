use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::string::ToString;

#[derive(Debug, Copy, PartialEq)]
pub enum Num {
    NaN,
    F64(f64),
}

impl Clone for Num {
    fn clone(&self) -> Self {
        use Num::*;
        match self {
            NaN => NaN,
            F64(x) => F64(*x),
        }
    }
}

impl ToString for Num {
    fn to_string(&self) -> String {
        use Num::*;
        match self {
            NaN => String::from("NaN"),
            F64(num) => num.to_string(),
        }
    }
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Num) -> Self::Output {
        use Num::*;
        match (self, rhs) {
            (F64(l), F64(r)) => F64(l + r),
            _ => NaN,
        }
    }
}

impl Sub for Num {
    type Output = Num;

    fn sub(self, rhs: Num) -> Self::Output {
        use Num::*;
        match (self, rhs) {
            (F64(l), F64(r)) => F64(l - r),
            _ => NaN,
        }
    }
}

#[derive(Debug)]
pub enum Type<'a> {
    Null,
    Undefined,
    Number(Num),
    Boolean(bool),
    TextString(String),
    Object(&'a Box<HashMap<String, Type<'a>>>),
    Function(String),
}

impl<'a> Add for Type<'a> {
    type Output = Type<'a>;

    fn add(self, rhs: Type) -> Self::Output {
        use Type::*;
        match self {
            Null => panic!("Not implemented!"),
            Undefined => match rhs {
                Number(_) => Number(Num::NaN),
                _ => panic!("Not implemented!"),
            },
            Number(lhs) => match rhs {
                Number(r) => Number(lhs + r),
                TextString(r) => {
                    let mut l = lhs.to_string();
                    l.push_str(&r);
                    TextString(l)
                }
                _ => panic!("Not implemented!"),
            },
            Boolean(boolean) => Boolean(boolean),
            TextString(lhs) => match rhs {
                TextString(r) => {
                    let mut l = lhs;
                    l.push_str(&r);
                    TextString(l)
                }
                Number(r) => {
                    let mut l = lhs;
                    l.push_str(&r.to_string());
                    TextString(l)
                }
                _ => panic!("Not implemented!"),
            },
            Object(object) => Object(object),
            Function(name) => Function(String::from(name)),
        }
    }
}

impl<'a> Sub for Type<'a> {
    type Output = Type<'a>;

    fn sub(self, rhs: Type) -> Self::Output {
        use Type::*;
        match self {
            Null => panic!("Not implemented!"),
            Undefined => match rhs {
                Number(_) => Number(Num::NaN),
                _ => panic!("Not implemented!"),
            },
            Number(lhs) => match rhs {
                Number(r) => Number(lhs - r),
                TextString(r) => {
                    let mut l = lhs.to_string();
                    l.push_str(&r);
                    TextString(l)
                }
                _ => panic!("Not implemented!"),
            },
            Boolean(boolean) => Boolean(boolean),
            TextString(lhs) => match rhs {
                TextString(r) => Number(Num::NaN),
                Number(r) => {
                    let mut l = lhs;
                    l.push_str(&r.to_string());
                    TextString(l)
                }
                _ => panic!("Not implemented!"),
            },
            Object(object) => Object(object),
            Function(name) => Function(String::from(name)),
        }
    }
}

impl<'a> Clone for Type<'a> {
    fn clone(&self) -> Self {
        use Type::*;
        match self {
            Null => Null,
            Undefined => Undefined,
            Number(number) => Number(*number),
            Boolean(boolean) => Boolean(*boolean),
            TextString(string) => TextString(String::from(string)),
            Object(object) => Object(*object),
            Function(name) => Function(String::from(name)),
        }
    }
}

#[derive(Debug)]
pub struct Function<'a> {
    arguments: HashMap<String, Type<'a>>,
    body: String,
}
