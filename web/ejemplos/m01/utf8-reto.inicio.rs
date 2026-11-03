// Cambia SOLO el texto para que imprima: 3 caracteres en 5 bytes
// Pista: cada «ñ» ocupa 2 bytes.
fn main() {
    let texto = "uno";
    println!("{} caracteres en {} bytes", texto.chars().count(), texto.len());
}
