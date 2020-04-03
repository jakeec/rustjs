use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug)]
pub enum Type<'a> {
    Null,
    Undefined,
    Number(f64),
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
            Number(number) => match rhs {
                Number(num) => Number(number + num),
                _ => panic!("Not implemented!"),
            },
            Boolean(boolean) => Boolean(boolean),
            TextString(string) => TextString(String::from(string)),
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
