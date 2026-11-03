use std::fmt::Display;

// T: ?Sized acepta también tipos sin tamaño fijo, como str o [i32]
fn mostrar<T: Display + ?Sized>(x: &T) {
    println!("[{x}] ocupa {} bytes", size_of_val(x));
}

fn main() {
    mostrar(&42);
    mostrar("hola"); // T = str
    let s = String::from("ñu");
    mostrar(s.as_str());
    let v: &[u8] = &[1, 2, 3];
    println!("{}", size_of_val(v));
}
