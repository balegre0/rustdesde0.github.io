// Estos 2 bytes llegan por la red, que usa big-endian.
// Cambia SOLO la función de lectura para que imprima: puerto 8080
fn main() {
    let bytes = [0x1f, 0x90];
    let puerto = u16::from_be_bytes(bytes);
    println!("puerto {puerto}");
}
