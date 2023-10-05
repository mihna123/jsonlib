pub mod parser;

use std::collections::HashMap;

pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    String(String),
    Number(Number),
    Bool(bool),
    Null
}

pub enum Number {
    Integer(i32),
    Real(f64)
}
