fn main() {
    let mut contador = 0;
    let mut sumar = || contador += 1;
    sumar();
    // El closure ya no se usa: su préstamo exclusivo terminó.
    println!("después: {contador}");
}
