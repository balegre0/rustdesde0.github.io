use std::fmt;

// Patrón newtype: un struct NUESTRO que envuelve el Vec<u8>.
struct Bytes(Vec<u8>);

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} bytes", self.0.len())
    }
}

fn main() {
    let v = Bytes(vec![1, 2, 3]);
    println!("{v}");
}
