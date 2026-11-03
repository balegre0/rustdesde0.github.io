fn doble(n: i32) -> i32 {
    n * 2
}

fn main() {
    let texto = "21";
    let n: i32 = texto.parse().unwrap(); // convertir el texto en número
    println!("{}", doble(n));
}
