use std::fmt;

struct Bytes(Vec<u8>); // tipo nuevo y local: le puedes implementar lo que quieras

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} bytes", self.0.len())
    }
}

fn main() {
    println!("{}", Bytes(vec![1, 2, 3]));
}
