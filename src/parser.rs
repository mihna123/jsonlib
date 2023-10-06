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

    pub fn get_token(&mut self) -> Option<&Token> {
        if self.index >= self.token_stream.len() {
            return None;
        }
        let tok = &self.token_stream[self.index];
        self.index += 1;

        Some(tok)
    }

    pub fn parse(&mut self) {
        todo!()
    }
}
