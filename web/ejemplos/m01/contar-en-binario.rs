// Contar del 0 al 8 en decimal y en binario.
fn main() {
    // 1. {:08b} = en binario, con 8 cifras, rellenando con ceros
    for n in 0..=8 {
        println!("{n} -> {:08b}", n);
    }

    // 2. También puedes ESCRIBIR números en binario con 0b
    let trece = 0b0000_1101;
    println!("0b0000_1101 vale {trece}");

    // 3. Sin el 08, Rust no pone ceros a la izquierda
    println!("{:b}", 5);
}
