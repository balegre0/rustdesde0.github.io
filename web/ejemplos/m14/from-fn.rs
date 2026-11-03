use std::iter;

fn main() {
    // 1. iter::from_fn: un iterador hecho con un closure FnMut que guarda estado.
    let (mut a, mut b) = (0u64, 1u64);
    let fibonacci = iter::from_fn(move || {
        let actual = a;
        (a, b) = (b, a + b);
        Some(actual)
    });
    let primeros: Vec<u64> = fibonacci.take(10).collect();
    println!("fibonacci: {primeros:?}");

    // 2. iter::successors: cada elemento se calcula a partir del anterior.
    let potencias: Vec<u32> = iter::successors(Some(1u32), |&x| x.checked_mul(10)).collect();
    println!("potencias de 10 que caben en u32: {potencias:?}");

    // 3. iter::repeat + take, y iter::once.
    let linea: String = iter::repeat('-').take(8).chain(iter::once('>')).collect();
    println!("{linea}");
}
