pub mod input_stream;
pub mod token;

use input_stream::InputStream;
use token::Token;

pub struct Tokenizer {
    stream: InputStream,
    line_number: usize,
    char_number: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            stream: InputStream::new(input),
            line_number: 1,
            char_number: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res = vec![];
        loop {
            let c: char;
            match self.stream.get_char() {
                Some(character) => {
                    c = character;
                    self.char_number += 1;
                }
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
                '\n' => {
                    self.char_number = 0;
                    self.line_number += 1;
                }
                //all whitespace
                '\u{0009}' |
                '\u{000b}' |
                '\u{000c}' |
                '\u{000d}' |
                '\u{0020}' |
                '\u{0085}' |
                '\u{00a0}' |
                '\u{1680}' |
                '\u{180e}' |
                '\u{2000}' |
                '\u{2001}' |
                '\u{2002}' |
                '\u{2003}' |
                '\u{2004}' |
                '\u{2005}' |
                '\u{2006}' |
                '\u{2007}' |
                '\u{2008}' |
                '\u{2009}' |
                '\u{200a}' |
                '\u{200b}' |
                '\u{200c}' |
                '\u{200d}' |
                '\u{2028}' |
                '\u{2029}' |
                '\u{202f}' |
                '\u{205f}' |
                '\u{2060}' |
                '\u{3000}' |
                '\u{feff}' => {}

                _ => res.push(Token::BadToken {
                    line_number: self.line_number,
                    char_number: self.char_number,
                }),
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
                ' ' | ',' | '}' | ']' => {
                    self.stream.unseek();
                    break;
                }
                _ => {
                    tokens.push(Token::BadToken {
                        line_number: self.line_number,
                        char_number: self.char_number,
                    });
                    return;
                }
            }
        }
        let number: f64 = number.parse().unwrap();
        tokens.push(Token::Number { value: number });
    }

    fn handle_true(&mut self, tokens: &mut Vec<Token>) {
        //TODO: fix the unwraping
        let mut next: char;
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 'r' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        }
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 'u' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        }
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 'e' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        }

        tokens.push(Token::True);
    }

    fn handle_false(&mut self, tokens: &mut Vec<Token>) {
        //TODO: fix the unwraping
        let mut next: char;
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 'a' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        }
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 'l' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        }
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 's' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        }
        next = if let Some(character) = self.stream.get_char() {
            character
        } else {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
            return;
        };
        if next != 'e' {
            tokens.push(Token::BadToken {
                line_number: self.line_number,
                char_number: self.char_number,
            });
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
                None => {
                    tokens.push(Token::BadToken {
                        line_number: self.line_number,
                        char_number: self.char_number,
                    });
                    return;
                }
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
    fn test_bad_string() {
        let mut tokenizer = Tokenizer::new("\"This is a bad string");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::BadToken {
                line_number: 1,
                char_number: 1
            }],
        );
    }

    #[test]
    fn test_bad_number() {
        let mut tokenizer = Tokenizer::new("342d");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::BadToken {
                line_number: 1,
                char_number: 1
            }],
        );
    }

    #[test]
    fn test_bad_false() {
        let mut tokenizer = Tokenizer::new("falsf");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::BadToken {
                line_number: 1,
                char_number: 1,
            }]
        );
    }

    #[test]
    fn test_bad_true() {
        let mut tokenizer = Tokenizer::new("trud");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::BadToken {
                line_number: 1,
                char_number: 1,
            }]
        );
    }

    #[test]
    fn test_line_num() {
        let mut tokenizer = Tokenizer::new("hey\nbro");
        tokenizer.tokenize();
        assert_eq!(tokenizer.line_number, 2);
        assert_eq!(tokenizer.char_number, 3);
    }

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
