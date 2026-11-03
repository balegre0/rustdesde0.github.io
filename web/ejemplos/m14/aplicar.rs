// 1. Genérico con bound: una copia de `aplicar` por cada closure distinto.
fn aplicar<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// 2. Lo mismo con `impl Fn` en el argumento (azúcar del genérico).
fn aplicar_dos_veces(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// 3. Con `where`, útil cuando hay varios bounds.
fn combinar<F, G>(f: F, g: G, x: i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    g(f(x))
}

fn incrementar(n: i32) -> i32 {
    n + 1
}

fn main() {
    println!("{}", aplicar(|n| n * 10, 4));
    println!("{}", aplicar_dos_veces(|n| n * n, 3));
    println!("{}", combinar(|n| n - 1, |n| n * 2, 6));
    // 4. Una función normal también implementa Fn: se pasa por su nombre.
    println!("{}", aplicar(incrementar, 41));
}
