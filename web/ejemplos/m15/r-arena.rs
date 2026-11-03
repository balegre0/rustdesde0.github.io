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
    // Amistades en los dos sentidos: un ciclo, pero de números.
    red[0].amigos.push(1);
    red[1].amigos.push(0);
    red[1].amigos.push(2);
    for p in &red {
        let nombres: Vec<&str> = p.amigos.iter().map(|&i| red[i].nombre).collect();
        println!("{}: {}", p.nombre, nombres.join(", "));
    }
}
