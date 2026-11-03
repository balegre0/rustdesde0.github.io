fn es_vocal(c: char) -> bool {
    matches!(c.to_lowercase().next().unwrap(),
        'a' | 'e' | 'i' | 'o' | 'u' | 'á' | 'é' | 'í' | 'ó' | 'ú' | 'ü')
}

fn main() {
    let frase = "El pingüino comió atún";
    let n = frase.chars().filter(|&c| es_vocal(c)).count();
    println!("{} vocales", n);
}
