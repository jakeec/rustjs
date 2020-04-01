use std::collections::HashMap;

#[derive(Debug)]
pub enum Types {
    Null,
    Undefined,
    Number(f64),
    Boolean(bool),
    TextString(String),
    Object(HashMap<String, Types>),
    Function(String),
}

#[derive(Debug)]
pub struct Function {
    arguments: HashMap<String, Types>,
    body: String,
}
