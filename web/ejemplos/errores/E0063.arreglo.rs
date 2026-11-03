struct Jugador {
    nombre: String,
    vidas: u8,
}

fn main() {
    let j = Jugador { nombre: String::from("Ana"), vidas: 3 };
    println!("{} {}", j.nombre, j.vidas);
}
