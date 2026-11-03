fn medir(datos: Box<[u8; 100_000]>) -> usize {
    // 2. Aquí `datos` es el nuevo dueño. Solo se copió el puntero.
    datos.len()
}

fn main() {
    // 1. 100 000 bytes en el heap; en la pila, solo un puntero.
    let grande = Box::new([7u8; 100_000]);
    let n = medir(grande);
    println!("medidos {n} bytes");
    // 3. `grande` se movió a medir: ya no se puede usar aquí.
    // println!("{}", grande[0]); // error E0382
}
