fn main() {
    let nombres = [String::from("Ana"), String::from("Luis")];
    let primero = &nombres[0]; // préstamo compartido del elemento
    println!("{primero}");
}
