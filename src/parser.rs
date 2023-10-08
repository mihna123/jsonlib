mod parser_state;

use crate::tokenizer::{token::Token, Tokenizer};
use crate::value::Value;
use parser_state::ParserState;
use std::collections::HashMap;

pub struct Parser {
    token_stream: Vec<Token>,
    index: usize,
    state: ParserState,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            token_stream: vec![],
            index: 0,
            state: ParserState::Idle,
        }
    }

    pub fn read_into_stream(&mut self, input: &str) {
        let mut tokenizer = Tokenizer::new(input);
        self.token_stream = tokenizer.tokenize();
    }

    fn get_token(&mut self) -> Option<Token> {
        if self.index >= self.token_stream.len() {
            return None;
        }
        let tok = self.token_stream[self.index].clone();
        self.index += 1;

        Some(tok)
    }

    pub fn parse(&mut self) -> Value {
        let mut res = Value::Object(HashMap::new());
        let mut val_name = String::new();

        loop {
            let tok: Token;
            match self.get_token() {
                Some(token) => tok = token,
                None => {
                    break;
                }
            }
            match self.state {
                ParserState::Idle => {
                    match tok {
                        Token::String { value } => {
                            val_name = value.clone();
                            self.state = ParserState::GotName;
                        }
                        Token::Number { value } => {
                            return Value::Number(value);
                        }
                        Token::OpenCurlyBrace => { /*Ignore*/ }
                        Token::ClosedCurlyBrace => {
                            return res;
                        }
                        Token::OpenSquareBrace => {}
                        Token::ClosedSquareBrace => {}
                        Token::Colon => { /*Err*/ }
                        Token::Comma => { /*Err*/ }
                        Token::True => {
                            return Value::Bool(true);
                        }
                        Token::False => {
                            return Value::Bool(false);
                        }
                        Token::Null => {
                            return Value::Null;
                        }
                    }
                }
                ParserState::GotName => {
                    match tok {
                        Token::Colon => {
                            self.state = ParserState::GotColon;
                        }
                        _ => { /*This is err*/ }
                    }
                }
                ParserState::GotColon => {
                    match tok {
                        Token::String { value } => {
                            let val = Value::String(value);
                            if let Value::Object(hm) = &mut res {
                                hm.insert(val_name.clone(), val);
                            }
                        }
                        Token::Number { value } => {
                            let val = Value::Number(value);
                            if let Value::Object(hm) = &mut res {
                                hm.insert(val_name.clone(), val);
                            }
                        }
                        Token::OpenCurlyBrace => {
                            self.state = ParserState::Idle;
                            let val = self.parse();
                            if let Value::Object(hm) = &mut res {
                                hm.insert(val_name.clone(), val);
                            }
                        }
                        Token::ClosedCurlyBrace => { /*This is err*/ }
                        Token::OpenSquareBrace => {}
                        Token::ClosedSquareBrace => {}
                        Token::Colon => { /*This is err*/ }
                        Token::Comma => { /*This is err*/ }
                        Token::True => {
                            if let Value::Object(hm) = &mut res {
                                hm.insert(val_name.clone(), Value::Bool(true));
                            }
                        }
                        Token::False => {
                            if let Value::Object(hm) = &mut res {
                                hm.insert(val_name.clone(), Value::Bool(false));
                            }
                        }
                        Token::Null => {
                            if let Value::Object(hm) = &mut res {
                                hm.insert(val_name.clone(), Value::Null);
                            }
                        }
                    }
                    self.state = ParserState::GotValue;
                }
                ParserState::GotValue => {
                    match tok {
                        Token::Comma => {
                            self.state = ParserState::Idle;
                        }
                        Token::ClosedCurlyBrace => {
                            return res;
                        }
                        _ => { /*error*/ }
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_simple_json() {
        let mut parser = Parser::new();
        parser.read_into_stream("{\"name\":\"Wazowski\"}");
        let res = parser.parse();
        if let Value::Object(map) = res {
            assert_eq!(map["name"], Value::String("Wazowski".to_string()));
        }
    }

    #[test]
    fn test_simple_json_2() {
        let mut parser = Parser::new();
        parser.read_into_stream(
            "{\
                                    \"name\": \"Mike\", \
                                    \"age\": 21, \
                                    \"alive\": true\
                                 }",
        );
        let res = parser.parse();
        if let Value::Object(map) = res {
            assert_eq!(map["name"], Value::String("Mike".to_string()));
            assert_eq!(map["age"], Value::Number(21.0));
            assert_eq!(map["alive"], Value::Bool(true));
        }
    }

    #[test]
    fn test_nested_json_1() {
        let mut parser = Parser::new();
        parser.read_into_stream(
            "{\
                                    \"object\": {\
                                                    \"name\": \"Mike\"\
                                                }\
                                 }",
        );
        let res = parser.parse();
        if let Value::Object(map) = res {
            let obj = &map["object"];
            if let Value::Object(object) = obj {
                println!("{:?}", object);
                assert_eq!(object["name"], Value::String("Mike".to_string()));
            }
        }
    }
}
