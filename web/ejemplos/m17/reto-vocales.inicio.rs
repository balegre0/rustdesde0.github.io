use std::thread;

fn contar_vocales(t: &str) -> usize {
    t.chars().filter(|c| "aeiouáéíóú".contains(*c)).count()
}

fn main() {
    let textos = ["murciélago", "ñandú", "ola"];
    // Usa thread::scope para contar las vocales de cada texto en su propio hilo.
    // Guarda los resultados EN ORDEN en `cuentas`.
    let cuentas: Vec<usize> = thread::scope(|_s| Vec::new());
    let _ = contar_vocales;
    for (t, n) in textos.iter().zip(&cuentas) {
        println!("{t}: {n}");
    }
}
