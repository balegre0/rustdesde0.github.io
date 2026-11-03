fn main() {
    let orden = String::from("salir");
    match orden {
        "salir" => println!("adiós"),
        _ => println!("no entiendo"),
    }
}
