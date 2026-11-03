use std::fmt;

// Regla del huérfano: no puedes implementar un trait ajeno (Display)
// para un tipo ajeno (Vec): al menos uno de los dos debe ser de tu crate.
impl fmt::Display for Vec<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} bytes", self.len())
    }
}

fn main() {}
