fn main() {
    let numeros = [0, 3, 8, 15];
    for n in numeros {
        let texto = match n {
            0 => "cero",
            1 | 2 | 3 => "poco",
            4..=9 => "bastante",
            _ => "mucho",
        };
        println!("{n}: {texto}");
    }
}
