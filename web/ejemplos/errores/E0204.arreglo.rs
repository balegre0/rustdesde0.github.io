#[derive(Clone)]
struct Usuario {
    nombre: String,
    edad: u32,
}

fn main() {
    let u = Usuario { nombre: String::from("Ana"), edad: 30 };
    let v = u.clone();
    println!("{} {}", u.edad, v.nombre);
}
