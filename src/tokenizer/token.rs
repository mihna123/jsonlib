pub enum Token {
    String {
        value: String
    },
    Number {
        value: f64
    },
    Colon,
    OpenCurlyBrace,
    ClosedCurlyBrace,
    True,
    False,
    Null,
    WhiteSpace
}
