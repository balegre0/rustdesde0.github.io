struct Jugador {
    nombre: String,
}

fn main() {
    let j = Jugador { nombre: String::from("Ana"), edad: 30 };
    println!("{}", j.nombre);
}
