fn main() {
    let mut puntos = 10;
    {
        let a = &mut puntos;
        *a += 1;
    } // el préstamo `a` termina aquí
    let b = &mut puntos;
    *b += 1;
    println!("{puntos}");
}
