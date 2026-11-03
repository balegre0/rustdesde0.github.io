// Cambia SOLO los bits para que imprima: 11111101 como i8 = -3
fn main() {
    // 3 = 00000011 -> invertir = 11111100 -> sumar 1 = 11111101
    let bits: u8 = 0b1111_1101;
    println!("{:08b} como i8 = {}", bits, bits as i8);
}
