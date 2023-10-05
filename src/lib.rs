pub mod parser;
pub mod tokenizer;

use std::collections::HashMap;

pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    String(String),
    Number(f64),
    Bool(bool),
    Null
}
