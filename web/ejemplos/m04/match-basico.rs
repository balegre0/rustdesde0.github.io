fn main() {
    for dado in [1, 3, 6, 4] {
        // 1. `match` compara el valor con cada patrón, de arriba abajo
        let texto = match dado {
            1 => "uno: ¡pierdes el turno!",
            // 2. `|` junta varios patrones en un brazo
            2 | 3 => "poco",
            // 3. `4..=5` es un patrón de rango
            4..=5 => "bien",
            6 => "¡seis! tira otra vez",
            // 4. `_` atrapa todo lo demás (aquí, valores imposibles del dado)
            _ => "dado trucado",
        };
        println!("{dado} -> {texto}");
    }

    // 5. `match` también funciona con bool, char y texto
    let letra = 'ñ';
    let es_vocal = match letra {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    };
    println!("¿'{letra}' es vocal? {es_vocal}");
}
