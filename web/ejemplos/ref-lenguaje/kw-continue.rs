fn main() {
    // 1. Saltar al siguiente paso del bucle.
    for n in 1..=10 {
        if n % 3 != 0 { continue; }
        println!("{n} es múltiplo de 3");
    }
    // 2. Con etiqueta: pasar al siguiente paso del bucle exterior.
    'filas: for fila in 0..3 {
        for col in 0..3 {
            if col > fila { continue 'filas; }
            print!("({fila},{col}) ");
        }
    }
    println!();
}
