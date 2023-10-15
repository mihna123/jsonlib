mod parser;
mod tokenizer;
pub mod value;

use parser::Parser;
use std::error::Error;
use value::Value;

pub fn parse(input: &str) -> Result<Value, Box<dyn Error>> {
    let mut parser = Parser::new();
    parser.read_into_stream(input);
    parser.parse_obj()
}
