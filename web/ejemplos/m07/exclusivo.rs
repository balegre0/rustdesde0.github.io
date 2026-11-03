fn duplicar(n: &mut i32) {
    *n *= 2; // escribir a través del préstamo: hay que seguir la flecha con *
}

fn main() {
    let mut puntos = 21;
    println!("antes: {puntos}");
    duplicar(&mut puntos);
    println!("después: {puntos}");

    let mut lista = vec![1, 2];
    let r = &mut lista;
    r.push(3); // con métodos no hace falta *: el punto sigue la flecha
    println!("{lista:?}");
}
