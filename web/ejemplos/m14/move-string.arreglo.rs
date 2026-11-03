fn main() {
    let s = String::from("hola");
    let copia = s.clone();
    let f = move || println!("dentro: {copia}");
    f();
    println!("fuera: {s}");
}
