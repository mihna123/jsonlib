use crate::tokenizer::{ Tokenizer, token::Token };

pub struct Parser {
    token_stream: Vec<Token>,
    index: usize
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            token_stream: vec![],
            index: 0
        }
    }

    pub fn read_into_stream(&mut self, input: &str) {
        let mut tokenizer = Tokenizer::new(input);
        self.token_stream = tokenizer.tokenize();
    }

    pub fn parse(&mut self) {
        todo!()
    }
}
