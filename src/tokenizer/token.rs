#[derive(Debug, PartialEq)]
pub enum Token {
    String {
        value: String
    },
    Number {
        value: f64
    },
    Colon,
    Comma,
    OpenCurlyBrace,
    ClosedCurlyBrace,
    True,
    False,
    Null,
    WhiteSpace
}
