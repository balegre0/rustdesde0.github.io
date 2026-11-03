fn palabras_largas(texto: &str, minimo: usize) -> Vec<String> {
    texto
        .split_whitespace()
        .filter(|p| p.chars().count() >= minimo)
        .map(|p| p.to_uppercase())
        .collect()
}
