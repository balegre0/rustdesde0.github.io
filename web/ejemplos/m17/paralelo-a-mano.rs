use std::thread;

// Aplica `f` a cada elemento usando `n` hilos y conserva el orden.
fn mapa_paralelo<T: Sync, R: Send>(datos: &[T], n: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let tam = datos.len().div_ceil(n).max(1);
    thread::scope(|s| {
        let manijas: Vec<_> = datos
            .chunks(tam)
            .map(|trozo| s.spawn(|| trozo.iter().map(&f).collect::<Vec<R>>()))
            .collect();
        manijas.into_iter().flat_map(|h| h.join().unwrap()).collect()
    })
}

fn main() {
    let n = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    println!("núcleos disponibles: {n}");
    let numeros: Vec<u64> = (1..=10).collect();
    let cuadrados = mapa_paralelo(&numeros, n, |x| x * x);
    println!("{cuadrados:?}");
}
