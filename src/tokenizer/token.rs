#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String { value: String },
    Number { value: f64 },
    Colon,
    Comma,
    OpenCurlyBrace,
    ClosedCurlyBrace,
    OpenSquareBrace,
    ClosedSquareBrace,
    True,
    False,
    Null,
    BadToken {
        line_number: usize,
        char_number: usize,
    },
}
