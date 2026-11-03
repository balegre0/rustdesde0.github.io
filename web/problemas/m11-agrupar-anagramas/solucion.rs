use std::collections::HashMap;

fn agrupar(palabras: &[&str]) -> Vec<Vec<String>> {
    let mut grupos: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for p in palabras {
        let mut firma: Vec<char> = p.chars().collect();
        firma.sort();
        grupos.entry(firma).or_default().push(p.to_string());
    }
    let mut resultado: Vec<Vec<String>> = grupos.into_values().collect();
    for g in &mut resultado {
        g.sort();
    }
    resultado.sort();
    resultado
}
