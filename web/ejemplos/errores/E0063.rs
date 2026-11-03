struct Jugador {
    nombre: String,
    vidas: u8,
}

fn main() {
    let j = Jugador { nombre: String::from("Ana") };
    println!("{}", j.nombre);
}
