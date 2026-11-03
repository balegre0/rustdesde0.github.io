// JEFE DEL MUNDO 11: el analizador de textos.
// Completa `analizar`. Debe devolver un Informe con:
//  - lineas: número de líneas
//  - palabras: número de palabras (separadas por espacios)
//  - letras: número de caracteres alfabéticos (c.is_alphabetic(); ¡las tildes y la ñ cuentan!)
//  - distintas: número de palabras distintas, en minúsculas y sin signos de puntuación
//    en los bordes (usa trim_matches(|c: char| !c.is_alphanumeric()) y to_lowercase())
//  - top: las 3 palabras más frecuentes (también normalizadas), de más a menos frecuente;
//    en caso de empate, por orden alfabético. Usa HashMap + entry y luego ordena un Vec.
//  - mas_larga: la palabra normalizada con más CARACTERES (no bytes); si empatan, la primera que aparezca.

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
    todo!()
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
