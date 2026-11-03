// Arreglo con lo que sabes hoy: la función devuelve la propiedad.
// (En el mundo 07 verás el arreglo de verdad: prestar con `&`.)
fn saludar(nombre: String) -> String {
    println!("¡Hola, {nombre}!");
    nombre
}

fn main() {
    let n = String::from("Ana");
    let n = saludar(n);
    println!("Adiós, {n}");
}
