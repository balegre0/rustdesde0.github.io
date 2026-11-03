use std::collections::{BTreeMap, BTreeSet, HashSet};

fn main() {
    // 1. BTreeMap: como HashMap, pero las claves están SIEMPRE ordenadas.
    let mut agenda: BTreeMap<&str, &str> = BTreeMap::new();
    agenda.insert("Marta", "600 111 222");
    agenda.insert("Álvaro", "600 333 444");
    agenda.insert("Bea", "600 555 666");
    for (nombre, tel) in &agenda {
        println!("{nombre:>7}: {tel}");
    }
    // Ojo: el orden es por bytes (UTF-8), así que «Á» va DESPUÉS de la «z».

    // 2. Rangos de claves y primero/último: solo en los mapas ordenados.
    let mut temperaturas: BTreeMap<u32, f64> = BTreeMap::new();
    for (hora, t) in [(6, 11.0), (9, 15.5), (12, 21.0), (15, 23.5), (18, 19.0), (21, 14.0)] {
        temperaturas.insert(hora, t);
    }
    println!("de 9 a 15 h: {:?}", temperaturas.range(9..=15).collect::<Vec<_>>());
    println!("primera: {:?}, última: {:?}", temperaturas.first_key_value(), temperaturas.last_key_value());

    // 3. HashSet: un conjunto. Cada valor, una vez. Responde «¿está?» al instante.
    let mut vistos = HashSet::new();
    for palabra in "la casa de la abuela de la sierra".split_whitespace() {
        if !vistos.insert(palabra) {
            println!("«{palabra}» repetida");
        }
    }
    println!("palabras distintas: {}", vistos.len());

    // 4. Operaciones de conjuntos (con BTreeSet para que salgan ordenadas).
    let rust: BTreeSet<&str> = ["Ana", "Luis", "Zoe", "Iker"].into_iter().collect();
    let web: BTreeSet<&str> = ["Luis", "Iker", "Marta"].into_iter().collect();
    println!("en los dos: {:?}", rust.intersection(&web).collect::<Vec<_>>());
    println!("en alguno: {:?}", rust.union(&web).collect::<Vec<_>>());
    println!("solo en rust: {:?}", rust.difference(&web).collect::<Vec<_>>());
}
