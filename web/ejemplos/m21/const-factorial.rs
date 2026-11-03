// `const fn` se puede ejecutar al compilar (con el intérprete de MIR) o en tiempo de ejecución.
const fn factorial(n: u64) -> u64 {
    let mut r = 1;
    let mut i = 2;
    while i <= n {
        r *= i;
        i += 1;
    }
    r
}

// 1. Calculado por el compilador: en el binario solo queda el número 3628800
const F10: u64 = factorial(10);

// 2. Una tabla entera construida en compilación
const CUADRADOS: [u32; 6] = {
    let mut t = [0; 6];
    let mut i = 0;
    while i < 6 {
        t[i] = (i * i) as u32;
        i += 1;
    }
    t
};

fn main() {
    println!("F10 = {F10}");
    println!("CUADRADOS = {CUADRADOS:?}");
    // 3. La misma función, ahora en tiempo de ejecución
    let n = CUADRADOS.len() as u64;
    println!("factorial({n}) en ejecución = {}", factorial(n));
    // 4. Un bloque const en línea: obliga a evaluar en compilación
    println!("en línea: {}", const { factorial(5) });
}
