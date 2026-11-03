use std::thread;

fn contar_vocales(t: &str) -> usize {
    t.chars().filter(|c| "aeiouáéíóú".contains(*c)).count()
}

fn main() {
    let textos = ["murciélago", "ñandú", "ola"];
    let cuentas: Vec<usize> = thread::scope(|s| {
        let manijas: Vec<_> = textos.iter().map(|t| s.spawn(move || contar_vocales(t))).collect();
        manijas.into_iter().map(|h| h.join().unwrap()).collect()
    });
    for (t, n) in textos.iter().zip(&cuentas) {
        println!("{t}: {n}");
    }
}
