use std::fmt::Display;

struct Punto { x: i32, y: i32 }

fn muestra<T: Display>(v: &[T]) {
    for x in v { println!("{x}"); }
}

fn main() {
    let ps = vec![Punto { x: 1, y: 2 }];
    muestra(&ps);
}
