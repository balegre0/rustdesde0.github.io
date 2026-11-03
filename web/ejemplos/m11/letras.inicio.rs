// Cuenta cuántas veces aparece cada letra (ignora los espacios) usando
// HashMap<char, usize> y la API entry. Luego imprime las letras ORDENADAS.

use std::collections::HashMap;

fn main() {
    let texto = "año nuevo";
    let mut cuenta: HashMap<char, usize> = HashMap::new();
    // escribe aquí el bucle

    let mut pares: Vec<_> = cuenta.into_iter().collect();
    pares.sort();
    println!("{:?}", pares);
}
