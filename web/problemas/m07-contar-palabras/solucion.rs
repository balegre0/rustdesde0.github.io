use std::collections::HashMap;

fn contar(texto: &str, mapa: &mut HashMap<String, u32>) {
    for palabra in texto.split_whitespace() {
        *mapa.entry(palabra.to_string()).or_insert(0) += 1;
    }
}
