mod texto {
    pub struct TextoAscii {
        bytes: Vec<u8>, // invariante: todos los bytes son ASCII
    }

    impl TextoAscii {
        pub fn nuevo(s: &str) -> Option<TextoAscii> {
            if s.is_ascii() { Some(TextoAscii { bytes: s.as_bytes().to_vec() }) } else { None }
        }

        pub fn empujar(&mut self, c: char) -> bool {
            if c.is_ascii() {
                self.bytes.push(c as u8);
                true
            } else {
                false
            }
        }

        pub fn como_str(&self) -> &str {
            // SAFETY: invariante del tipo: todos los bytes son ASCII, y el ASCII
            // es UTF-8 válido. Solo los métodos de este módulo tocan `bytes`.
            unsafe { std::str::from_utf8_unchecked(&self.bytes) }
        }
    }
}
use texto::TextoAscii;
