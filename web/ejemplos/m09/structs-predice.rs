struct Jugador {
    nombre: String,
    vidas: u32,
    puntos: u32,
}

fn main() {
    let ana = Jugador { nombre: String::from("Ana"), vidas: 3, puntos: 120 };
    // Un jugador nuevo que copia todo lo de `ana` salvo el nombre:
    let luis = Jugador { nombre: String::from("Luis"), ..ana };
    println!("{} tiene {} vidas y {} puntos", luis.nombre, luis.vidas, luis.puntos);
}
