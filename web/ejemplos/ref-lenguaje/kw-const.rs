// 1. Constante: un valor fijo, calculado al compilar.
const SEGUNDOS_POR_DIA: u32 = 24 * 60 * 60;

// 2. Función const: se puede evaluar al compilar (y también al ejecutar).
const fn cuadrado(x: u32) -> u32 {
    x * x
}
const AREA: u32 = cuadrado(12);

// 3. Parámetro genérico const: un número que forma parte del tipo.
fn suma<const N: usize>(v: [i32; N]) -> i32 {
    v.iter().sum()
}

fn main() {
    println!("{SEGUNDOS_POR_DIA}");
    println!("{AREA}");
    println!("{}", suma([1, 2, 3, 4]));
    // 4. Bloque const: obliga a calcular la expresión al compilar.
    let bits = const { u64::BITS * 2 };
    println!("{bits}");
    // 5. `*const T`: puntero crudo de solo lectura.
    let x = 5;
    let p: *const i32 = &x;
    // SAFETY: `p` apunta a `x`, que sigue viva.
    println!("{}", unsafe { *p });
}
