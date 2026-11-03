// 0x3f800000 son los bits de 1.0. Cambia SOLO el número hexadecimal
// para que imprima: 2
// Pista: 2.0 tiene el mismo signo y la misma mantisa; su exponente es uno más.
fn main() {
    let x = f32::from_bits(0x3f80_0000);
    println!("{x}");
}
