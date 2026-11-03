use std::fmt;

struct Punto {
    x: i32,
    y: i32,
}

impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Punto { x: 1, y: 2 };
    println!("{}", p);
}
