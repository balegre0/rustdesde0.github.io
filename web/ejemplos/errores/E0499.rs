fn main() {
    let mut puntos = 10;
    let a = &mut puntos;
    let b = &mut puntos;
    *a += 1;
    *b += 1;
    println!("{puntos}");
}
