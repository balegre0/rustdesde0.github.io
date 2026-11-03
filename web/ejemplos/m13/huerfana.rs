use std::fmt;

// Ni Display ni Vec<u8> son nuestros: los dos vienen de la biblioteca estándar.
impl fmt::Display for Vec<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} bytes", self.len())
    }
}

fn main() {
    let v: Vec<u8> = vec![1, 2, 3];
    println!("{v}");
}
