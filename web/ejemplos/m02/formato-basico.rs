fn main() {
    // 1. {} muestra el valor «para personas».
    println!("{}", "hola");
    // 2. {:?} lo muestra «para programadores»: se ven las comillas.
    println!("{:?}", "hola");
    // 3. Con {0}, {1}… eliges qué argumento va en cada hueco.
    println!("{0} y {1}, {1} y {0}", "pan", "vino");
    // 4. También puedes poner nombre a un argumento.
    println!("{ciudad} está en {pais}", ciudad = "Lima", pais = "Perú");
    // 5. Para escribir llaves de verdad, se duplican.
    println!("{{}} son dos llaves");
}
