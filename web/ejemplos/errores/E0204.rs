#[derive(Clone, Copy)]
struct Usuario {
    nombre: String,
    edad: u32,
}

fn main() {
    let u = Usuario { nombre: String::from("Ana"), edad: 30 };
    let v = u;
    println!("{} {}", u.edad, v.nombre);
}
