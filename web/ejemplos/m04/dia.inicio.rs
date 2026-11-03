fn main() {
    for dia in [1, 5, 6, 7, 9] {
        // Completa el `match`: 1 a 5 -> "laborable", 6 o 7 -> "fin de semana",
        // cualquier otro número -> "no existe"
        let tipo = match dia {
            _ => "?",
        };
        println!("día {dia}: {tipo}");
    }
}
