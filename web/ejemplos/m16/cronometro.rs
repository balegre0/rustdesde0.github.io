// Un «benchmark» casero: medir cuánto tarda algo. Los números cambian en
// cada ejecución y en cada máquina; fíjate en la proporción, no en el valor.
use std::hint::black_box;
use std::time::Instant;

fn suma_bucle(n: u64) -> u64 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += i;
        i += 1;
    }
    s
}

fn suma_formula(n: u64) -> u64 {
    // Gauss: 0 + 1 + ... + (n-1) = n(n-1)/2
    n * (n - 1) / 2
}

fn main() {
    let n = black_box(1_000_000);

    // 1. Repetimos muchas veces para que el tiempo sea medible.
    let inicio = Instant::now();
    for _ in 0..100 {
        black_box(suma_bucle(black_box(n)));
    }
    let t_bucle = inicio.elapsed();

    // 2. Lo mismo con la fórmula.
    let inicio = Instant::now();
    for _ in 0..100 {
        black_box(suma_formula(black_box(n)));
    }
    let t_formula = inicio.elapsed();

    // 3. Antes de comparar velocidades, comprueba que dan lo mismo.
    assert_eq!(suma_bucle(n), suma_formula(n));
    println!("bucle:   {t_bucle:?}");
    println!("fórmula: {t_formula:?}");
}
