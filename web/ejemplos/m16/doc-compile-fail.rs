// Esto es lo que comprueba un doctest marcado con `compile_fail`:
// que el código de ejemplo NO compile. Aquí, usar una String tras moverla.
fn main() {
    let nombre = String::from("Ñandú");
    let otro = nombre;
    println!("{nombre} y {otro}");
}
