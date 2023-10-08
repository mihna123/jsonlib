pub enum ParserState {
    Idle,
    GotName,
    GotValue,
    GotColon,
}
