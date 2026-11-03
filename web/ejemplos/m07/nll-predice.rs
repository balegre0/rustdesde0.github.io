fn main() {
    let mut puntos = vec![10, 20];
    let ultimo = puntos[puntos.len() - 1];
    let total = &mut puntos;
    total.push(ultimo + 10);
    let vista = &puntos;
    println!("{vista:?} {}", vista.len());
}
