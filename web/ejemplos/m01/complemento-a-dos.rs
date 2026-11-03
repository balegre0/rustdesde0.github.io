// Complemento a dos: así guarda la máquina los enteros con signo.
fn main() {
    // 1. Algunos i8 y sus 8 bits
    for n in [0i8, 1, 2, 127, -1, -2, -128] {
        println!("{n:>4} -> {:08b}", n);
    }

    // 2. Para cambiar de signo: invertir todos los bits (!) y sumar 1
    let cinco: i8 = 5;
    let invertido = !cinco;
    println!("5        = {:08b}", cinco);
    println!("!5       = {:08b}", invertido);
    println!("!5 + 1   = {:08b} = {}", invertido + 1, invertido + 1);

    // 3. Los mismos 8 bits, leídos sin signo y con signo
    let bits: u8 = 0b1100_1000;
    println!("{:08b} como u8 = {}, como i8 = {}", bits, bits, bits as i8);
}
