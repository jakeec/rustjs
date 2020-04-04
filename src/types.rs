use std::collections::HashMap;
use std::ops::Add;
use std::string::ToString;

#[derive(Debug, Copy)]
pub enum Number {
    NaN,
    F64(f64),
}

impl Clone for Number {
    fn clone(&self) -> Self {
        use Number::*;
        match self {
            NaN => NaN,
            F64(x) => F64(*x),
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        use Number::*;
        match self {
            NaN => String::from("NaN"),
            F64(num) => num.to_string(),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        use Number::*;
        match (self, rhs) {
            (F64(l), F64(r)) => F64(l + r),
            _ => NaN,
        }
    }
}

#[derive(Debug)]
pub enum Type<'a> {
    Null,
    Undefined,
    Number(Number),
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
            Null => Null,
            Undefined => Undefined,
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
