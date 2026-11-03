fn invertir_palabras(frase: &str) -> String {
    frase
        .split(' ')
        .map(|p| p.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
