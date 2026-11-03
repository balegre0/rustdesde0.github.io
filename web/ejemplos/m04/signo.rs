fn main() {
    for n in [7, -3, 0, 12] {
        let signo = if n > 0 {
            "positivo"
        } else if n < 0 {
            "negativo"
        } else {
            "cero"
        };
        println!("{n} es {signo}");
    }
}
