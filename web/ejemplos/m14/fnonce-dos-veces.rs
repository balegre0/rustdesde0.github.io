fn main() {
    let regalo = String::from("caja");
    let entregar = move || regalo;
    let a = entregar();
    let b = entregar();
    println!("{a} {b}");
}
