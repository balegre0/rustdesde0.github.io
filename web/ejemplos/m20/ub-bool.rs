// ⚠ Este programa tiene comportamiento indefinido. NO lo «arregles» mirando su salida:
// pásalo por Miri (Playground → Tools → Miri).
fn main() {
    let byte: u8 = 2;
    // ¡MAL! 2 no es un bool válido: producir este valor ya es UB,
    // aunque nunca lo usáramos.
    let b: bool = unsafe { std::mem::transmute::<u8, bool>(byte) };
    if b {
        println!("verdadero");
    } else {
        println!("falso");
    }
}
