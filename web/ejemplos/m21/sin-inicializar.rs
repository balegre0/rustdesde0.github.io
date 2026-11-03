fn main() {
    let x: i32;
    let llueve = true;
    if llueve {
        x = 1;
    }
    // En MIR hay un camino (llueve == false) en el que x nunca recibió valor.
    println!("{x}");
}
