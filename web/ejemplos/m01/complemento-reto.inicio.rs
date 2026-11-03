// Cambia SOLO los bits para que imprima: 11111101 como i8 = -3
fn main() {
    let bits: u8 = 0b0000_0011;
    println!("{:08b} como i8 = {}", bits, bits as i8);
}
