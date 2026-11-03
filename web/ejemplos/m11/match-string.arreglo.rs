fn main() {
    let orden = String::from("salir");
    match orden.as_str() {
        "salir" => println!("adiós"),
        _ => println!("no entiendo"),
    }
}
