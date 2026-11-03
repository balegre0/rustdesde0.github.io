fn main() {
    let mut contador = 0;
    let mut sumar = || contador += 1;
    println!("antes: {contador}");
    sumar();
}
