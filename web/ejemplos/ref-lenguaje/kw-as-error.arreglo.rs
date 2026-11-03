fn main() {
    let texto = "42";
    // Para leer un número de un texto se usa `parse`, que puede fallar.
    let n: i32 = texto.parse().expect("no es un número");
    println!("{n}");
}
