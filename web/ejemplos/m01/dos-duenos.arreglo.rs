fn main() {
    let a = String::from("hola");
    // Una copia de verdad: otro bloque del heap con otro dueño
    let b = a.clone();
    println!("{a} {b}");
}
