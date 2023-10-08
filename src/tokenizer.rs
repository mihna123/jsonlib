pub mod input_stream;
pub mod token;

use input_stream::InputStream;
use token::Token;

pub struct Tokenizer {
    stream: InputStream,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            stream: InputStream::new(input),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res = vec![];
        loop {
            let c: char;
            match self.stream.get_char() {
                Some(character) => c = character,
                None => break,
            }

            match c {
                '"' => self.handle_string(&mut res),
                ',' => res.push(Token::Comma),
                '{' => res.push(Token::OpenCurlyBrace),
                '}' => res.push(Token::ClosedCurlyBrace),
                '[' => res.push(Token::OpenSquareBrace),
                ']' => res.push(Token::ClosedSquareBrace),
                ':' => res.push(Token::Colon),
                't' => self.handle_true(&mut res),
                'f' => self.handle_false(&mut res),
                '0'..='9' => {
                    self.stream.unseek();
                    self.handle_number(&mut res);
                }
                _ => {}
            }
        }
        res
    }
    fn handle_number(&mut self, tokens: &mut Vec<Token>) {
        let mut number = String::new();
        let mut dot = false;
        loop {
            let c: char;
            match self.stream.get_char() {
                Some(character) => c = character,
                None => break,
            }
            match c {
                '0'..='9' => number.push(c),
                '.' if !dot => {
                    number.push(c);
                    dot = true;
                }
                _ => {
                    self.stream.unseek();
                    break;
                }
            }
        }
        let number: f64 = number.parse().unwrap();
        tokens.push(Token::Number { value: number });
    }

    fn handle_true(&mut self, tokens: &mut Vec<Token>) {
        //TODO: fix the unwraping
        let mut next = self.stream.get_char().unwrap();
        if next != 'r' {
            return;
        }
        next = self.stream.get_char().unwrap();
        if next != 'u' {
            return;
        }
        next = self.stream.get_char().unwrap();
        if next != 'e' {
            return;
        }

        tokens.push(Token::True);
    }

    fn handle_false(&mut self, tokens: &mut Vec<Token>) {
        //TODO: fix the unwraping
        let mut next = self.stream.get_char().unwrap();
        if next != 'a' {
            return;
        }
        next = self.stream.get_char().unwrap();
        if next != 'l' {
            return;
        }
        next = self.stream.get_char().unwrap();
        if next != 's' {
            return;
        }
        next = self.stream.get_char().unwrap();
        if next != 'e' {
            return;
        }

        tokens.push(Token::False);
    }

    fn handle_string(&mut self, tokens: &mut Vec<Token>) {
        let mut string_val = String::new();
        loop {
            let c: char;
            match self.stream.get_char() {
                Some(character) => c = character,
                None => break,
            }
            match c {
                '"' => break,
                _ => string_val.push(c),
            }
        }
        tokens.push(Token::String { value: string_val });
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_simple_string() {
        let mut tokenizer = Tokenizer::new("\"Hello World!\"");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::String {
                value: "Hello World!".to_string()
            }]
        )
    }

    #[test]
    fn test_simple_tokens() {
        let mut tokenizer = Tokenizer::new("{\"hi\"}");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::OpenCurlyBrace,
                Token::String {
                    value: "hi".to_string()
                },
                Token::ClosedCurlyBrace
            ]
        );
    }

    #[test]
    fn test_true_token() {
        let mut tokenizer = Tokenizer::new("\"true\":true");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::String {
                    value: "true".to_string()
                },
                Token::Colon,
                Token::True
            ]
        )
    }

    #[test]
    fn test_false_token() {
        let mut tokenizer = Tokenizer::new("\"false\":false");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::String {
                    value: "false".to_string()
                },
                Token::Colon,
                Token::False
            ]
        )
    }

    #[test]
    fn test_number_token() {
        let mut tokenizer = Tokenizer::new("420");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![Token::Number { value: 420.0 }]);
    }

    #[test]
    fn test_basic_json_tokenization() {
        let mut tokenizer = Tokenizer::new("{ \"age\" : 23, \"male\" : true }");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::OpenCurlyBrace,
                Token::String {
                    value: "age".to_string()
                },
                Token::Colon,
                Token::Number { value: 23.0 },
                Token::Comma,
                Token::String {
                    value: "male".to_string()
                },
                Token::Colon,
                Token::True,
                Token::ClosedCurlyBrace
            ]
        );
    }

    #[test]
    fn test_square_bracket_token() {
        let mut tokenizer = Tokenizer::new("[]");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::OpenSquareBrace, Token::ClosedSquareBrace]
        );
    }
}
