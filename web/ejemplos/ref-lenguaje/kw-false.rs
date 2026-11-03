fn main() {
    // 1. `false` y `true` son los dos únicos valores de `bool`.
    let lloviendo = false;
    println!("{}", !lloviendo);
    // 2. Como número: false = 0, true = 1.
    println!("{} {}", false as u8, true as u8);
    // 3. Ocupa un byte.
    println!("{} byte", size_of::<bool>());
    // 4. Operadores lógicos con cortocircuito.
    println!("{}", false && { println!("no se evalúa"); true });
    println!("{}", false || true);
}
