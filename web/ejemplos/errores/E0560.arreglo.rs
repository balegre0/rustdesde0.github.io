struct Jugador {
    nombre: String,
    edad: u32,
}

fn main() {
    let j = Jugador { nombre: String::from("Ana"), edad: 30 };
    println!("{} {}", j.nombre, j.edad);
}
