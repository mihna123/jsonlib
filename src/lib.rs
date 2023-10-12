mod parser;
mod tokenizer;
mod value;

use value::Value;
use parser::Parser;
use std::error::Error;

pub fn parse(input: &str) -> Result<Value, Box<dyn Error>> {
    let mut parser = Parser::new();
    parser.read_into_stream(input);
    parser.parse_obj()
}


