// Versión del simulador en la que `x += 1` es UN solo paso indivisible (atómico).
fn ejecutar(orden: &[usize]) -> i32 {
    let mut x = 0;
    // Completa: cada elemento de `orden` es un hilo que hace x += 1 de una vez.
    let _ = orden;
    x += 0;
    x
}

fn main() {
    // Con un paso por hilo solo hay 2 intercalaciones: H1 H2 y H2 H1.
    let ordenes = [[0, 1], [1, 0]];
    for o in ordenes {
        println!("{:?} -> x = {}", o, ejecutar(&o));
    }
}
