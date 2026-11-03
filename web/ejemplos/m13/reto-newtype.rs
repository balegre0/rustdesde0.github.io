use std::fmt;

struct Grados(f64);

impl fmt::Display for Grados {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°", self.0)
    }
}

fn main() {
    println!("{}", Grados(21.456));
    println!("{}", Grados(-3.0));
}
