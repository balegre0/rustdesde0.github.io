// Escribe en binario el número 42, para que imprima: 42 = 00101010
fn main() {
    // 42 = 32 + 8 + 2
    let n: u8 = 0b0010_1010;
    println!("{n} = {:08b}", n);
}
