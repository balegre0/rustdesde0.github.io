mod texto {
    pub struct TextoAscii {
        bytes: Vec<u8>, // invariante: todos los bytes son ASCII
    }

    impl TextoAscii {
        pub fn nuevo(s: &str) -> Option<TextoAscii> {
            todo!()
        }

        pub fn empujar(&mut self, c: char) -> bool {
            todo!()
        }

        pub fn como_str(&self) -> &str {
            todo!("from_utf8_unchecked con // SAFETY:")
        }
    }
}
use texto::TextoAscii;
