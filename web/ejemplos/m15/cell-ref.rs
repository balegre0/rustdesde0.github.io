use std::cell::Cell;

fn main() {
    let c = Cell::new(String::from("hola"));
    // Cell no presta lo de dentro: get() exige Copy, y String no lo es.
    let s = c.get();
    println!("{s}");
}
