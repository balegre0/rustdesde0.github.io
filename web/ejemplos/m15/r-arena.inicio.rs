// Una red de amigos sin Rc ni RefCell: todos viven en un Vec y se nombran por índice.
struct Persona {
    nombre: &'static str,
    amigos: Vec<usize>,
}

fn main() {
    let mut red: Vec<Persona> = Vec::new();
    for nombre in ["Ana", "Luis", "Sara"] {
        red.push(Persona { nombre, amigos: Vec::new() });
    }
    // Ana (0) y Luis (1) son amigos mutuos; Luis también es amigo de Sara (2).
    // Añade los índices a los Vec `amigos` correspondientes.
    todo!();
    for p in &red {
        let nombres: Vec<&str> = p.amigos.iter().map(|&i| red[i].nombre).collect();
        println!("{}: {}", p.nombre, nombres.join(", "));
    }
}
