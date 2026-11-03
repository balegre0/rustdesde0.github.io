// Las dos funciones del experimento de ensamblador, ahora ejecutándose.
fn suma_bucle(v: &[u32]) -> u32 {
    let mut s = 0u32;
    let mut i = 0;
    while i < v.len() {
        s = s.wrapping_add(v[i]);
        i += 1;
    }
    s
}

fn suma_iter(v: &[u32]) -> u32 {
    v.iter().fold(0u32, |a, &x| a.wrapping_add(x))
}

fn pares_cuadrados(n: u64) -> u64 {
    (0..n).filter(|x| x % 2 == 0).map(|x| x * x).sum()
}

fn main() {
    let datos: Vec<u32> = (1..=1000).collect();
    // 1. Mismo resultado...
    println!("bucle:     {}", suma_bucle(&datos));
    println!("iterador:  {}", suma_iter(&datos));
    // 2. ...y una cadena de adaptadores: 0² + 2² + 4² + 6² + 8².
    println!("pares²(10) = {}", pares_cuadrados(10));
}
