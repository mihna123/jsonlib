pub struct InputStream {
    pub buffer: Vec<u8>,
    pub index: usize
}

impl InputStream {
    pub fn new(input: &str) -> InputStream {
        InputStream {
            buffer: Vec::from(input.as_bytes()),
            index: 0
        }
    }

    pub fn get_char(&mut self) -> Option<char> {
        if self.index >= self.buffer.len() {
            return None;
        }

        let c = self.buffer[self.index];
        self.index += 1;
        Some(c as char)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_simple_input_string(){
        let mut stream = InputStream::new("Hello World!");
        assert_eq!(stream.get_char().unwrap(), 'H');
        assert_eq!(stream.get_char().unwrap(), 'e');
        assert_eq!(stream.get_char().unwrap(), 'l');
        assert_eq!(stream.get_char().unwrap(), 'l');
        assert_eq!(stream.get_char().unwrap(), 'o');
        assert_eq!(stream.get_char().unwrap(), ' ');
        assert_eq!(stream.get_char().unwrap(), 'W');
        assert_eq!(stream.get_char().unwrap(), 'o');
        assert_eq!(stream.get_char().unwrap(), 'r');
        assert_eq!(stream.get_char().unwrap(), 'l');
        assert_eq!(stream.get_char().unwrap(), 'd');
    }
    
    #[test]
    fn test_out_of_bounds(){
        let mut stream = InputStream::new("1");
        stream.get_char();
        assert!(stream.get_char().is_none());
    }
}
