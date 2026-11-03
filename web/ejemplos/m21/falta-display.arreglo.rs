use std::fmt::{self, Display};

struct Punto { x: i32, y: i32 }

// Le damos al solver la regla que le faltaba: Punto cumple Display.
impl Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn muestra<T: Display>(v: &[T]) {
    for x in v { println!("{x}"); }
}

fn main() {
    let ps = vec![Punto { x: 1, y: 2 }];
    muestra(&ps);
}
