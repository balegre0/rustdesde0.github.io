fn main() {
    for dia in [1, 5, 6, 7, 9] {
        let tipo = match dia {
            1..=5 => "laborable",
            6 | 7 => "fin de semana",
            _ => "no existe",
        };
        println!("día {dia}: {tipo}");
    }
}
