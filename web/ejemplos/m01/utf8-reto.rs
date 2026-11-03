// Cambia SOLO el texto para que imprima: 3 caracteres en 5 bytes
// Pista: cada «ñ» ocupa 2 bytes.
fn main() {
    let texto = "ñuñ";
    println!("{} caracteres en {} bytes", texto.chars().count(), texto.len());
}
