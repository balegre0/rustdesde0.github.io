fn main() {
    let s = String::from("hola");
    let f = move || println!("dentro: {s}");
    f();
    println!("fuera: {s}");
}
