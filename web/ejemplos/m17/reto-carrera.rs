// Versión del simulador en la que `x += 1` es UN solo paso indivisible (atómico).
fn ejecutar(orden: &[usize]) -> i32 {
    let mut x = 0;
    for _ in orden {
        x += 1; // leer, sumar y escribir de una vez: nadie se puede colar
    }
    x
}

fn main() {
    // Con un paso por hilo solo hay 2 intercalaciones: H1 H2 y H2 H1.
    let ordenes = [[0, 1], [1, 0]];
    for o in ordenes {
        println!("{:?} -> x = {}", o, ejecutar(&o));
    }
}
