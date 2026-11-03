#[derive(Clone, Copy)]
struct Usuario {
    edad: u32,
    nombre: String,
}

fn main() {
    let u = Usuario { edad: 30, nombre: String::from("Ana") };
    let v = u;
    println!("{} {}", u.edad, v.nombre);
}
