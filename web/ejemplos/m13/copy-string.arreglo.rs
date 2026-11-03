// Sin Copy: Clone basta, y la copia se pide a propósito con .clone().
#[derive(Clone)]
struct Usuario {
    edad: u32,
    nombre: String,
}

fn main() {
    let u = Usuario { edad: 30, nombre: String::from("Ana") };
    let v = u.clone();
    println!("{} {}", u.edad, v.nombre);
}
