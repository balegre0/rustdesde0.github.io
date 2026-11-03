struct Jugador {
    nombre: String,
    puntos: u32,
}

fn main() {
    let mut j = Jugador { nombre: String::from("Íñigo"), puntos: 0 };

    // El closure solo usa j.nombre: captura ESE campo, no todo `j`.
    let presentar = || println!("Juega {}", j.nombre);

    // Por eso podemos modificar otro campo mientras el closure vive.
    j.puntos += 10;
    presentar();
    println!("puntos = {}", j.puntos);
}
