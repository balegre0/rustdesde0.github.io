// Escribe `etiqueta(nombre: &str, edad: u32) -> String` que devuelva "nombre (edad)".
// Llámala con un literal y con un String.

fn etiqueta(nombre: &str, edad: u32) -> String {
    todo!()
}

fn main() {
    let propio = String::from("Lucía");
    println!("{}", etiqueta("Bruno", 8));
    println!("{}", etiqueta(&propio, 31));
}
