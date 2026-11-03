// Escribe en binario el número 42, para que imprima: 42 = 00101010
fn main() {
    let n: u8 = 0b0000_0000;
    println!("{n} = {:08b}", n);
}
