fn main() {
    let texto = "42";
    // `as` solo convierte entre tipos primitivos compatibles:
    // no sabe leer un número dentro de un texto.
    let n = texto as i32;
    println!("{n}");
}
