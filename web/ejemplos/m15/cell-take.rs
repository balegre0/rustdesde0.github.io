use std::cell::Cell;

fn main() {
    let nombre = Cell::new(String::from("Rut"));
    // take() saca el valor y deja el valor por defecto (un String vacío).
    let sacado = nombre.take();
    println!("sacado = «{sacado}»");
    println!("queda = «{}»", nombre.take());
}
