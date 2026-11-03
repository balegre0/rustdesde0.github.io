use std::mem::size_of;

// 1. Recibir &str: acepta literales, &String y trozos.
fn saludar(nombre: &str) -> String {
    format!("¡Hola, {nombre}!")
}

// 2. Guardar String en un struct: el struct es dueño de su texto.
struct Usuario {
    nombre: String,
}

fn main() {
    // 3. Un literal es un &str que vive en el binario del programa (&'static str).
    let literal: &'static str = "Begoña";

    // 4. String: texto propio en el heap, que puede crecer.
    let mut propio = String::from("Íñigo");
    propio.push_str(" Pérez");

    // 5. De String a &str: con & (coerción) o con as_str().
    println!("{}", saludar(literal));
    println!("{}", saludar(&propio));
    println!("{}", saludar(propio.as_str()));
    println!("{}", saludar(&propio[..6])); // un trozo de 6 BYTES: «Í» y «ñ» ocupan 2 cada una

    // 6. De &str a String: String::from, to_string, to_owned, into.
    let u = Usuario { nombre: literal.to_string() };
    let copia: String = "texto".into();
    println!("{} / {}", u.nombre, copia);

    // 7. Tamaños: String = puntero + capacidad + longitud; &str = puntero + longitud.
    println!("String: {} bytes, &str: {} bytes", size_of::<String>(), size_of::<&str>());
    println!("«{}» ocupa {} bytes de texto; capacidad {}", propio, propio.len(), propio.capacity());
}
