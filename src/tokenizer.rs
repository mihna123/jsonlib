pub mod token;
pub mod input_stream;

use token::Token;
use input_stream::InputStream;

pub struct Tokenizer {
    stream: InputStream
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            stream: InputStream::new(input)
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res = vec![];
        loop {
            let c: char;
            match self.stream.get_char() {
                Some(character) => { c = character },
                None => break
            }

            match c {
                '"' => {/*Handle string token*/}
                '{' => res.push(Token::OpenCurlyBrace),
                '}' => res.push(Token::ClosedCurlyBrace),
                ':' => res.push(Token::Colon),
                ' ' => {/*Handle whitespace*/}
                't' => {/*Handle posible true token*/}
                'f' => {/*Handle posible false token*/}
                '0' ..= '9' => {/*Handle posible number token*/}
                _ => {}
            }
        }
        res
    }
}
