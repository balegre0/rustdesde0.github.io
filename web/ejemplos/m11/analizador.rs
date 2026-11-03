// JEFE DEL MUNDO 11: el analizador de textos (solución).

use std::collections::HashMap;

#[derive(Debug)]
struct Informe {
    lineas: usize,
    palabras: usize,
    letras: usize,
    distintas: usize,
    top: Vec<(String, usize)>,
    mas_larga: String,
}

fn normalizar(palabra: &str) -> String {
    palabra.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase()
}

fn analizar(texto: &str) -> Informe {
    let mut cuenta: HashMap<String, usize> = HashMap::new();
    let mut palabras = 0;
    let mut mas_larga = String::new();
    for p in texto.split_whitespace() {
        palabras += 1;
        let n = normalizar(p);
        if n.chars().count() > mas_larga.chars().count() {
            mas_larga = n.clone();
        }
        *cuenta.entry(n).or_insert(0) += 1;
    }
    let mut top: Vec<(String, usize)> = cuenta.iter().map(|(k, v)| (k.clone(), *v)).collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    top.truncate(3);
    Informe {
        lineas: texto.lines().count(),
        palabras,
        letras: texto.chars().filter(|c| c.is_alphabetic()).count(),
        distintas: cuenta.len(),
        top,
        mas_larga,
    }
}

fn main() {
    let texto = "En un lugar de la Mancha, de cuyo nombre no quiero acordarme,\n\
                 no ha mucho tiempo que vivía un hidalgo de los de lanza en astillero,\n\
                 adarga antigua, rocín flaco y galgo corredor.";
    let inf = analizar(texto);
    println!("líneas: {}", inf.lineas);
    println!("palabras: {}", inf.palabras);
    println!("letras: {}", inf.letras);
    println!("distintas: {}", inf.distintas);
    println!("top 3: {:?}", inf.top);
    println!("más larga: {} ({} letras, {} bytes)", inf.mas_larga, inf.mas_larga.chars().count(), inf.mas_larga.len());
}
