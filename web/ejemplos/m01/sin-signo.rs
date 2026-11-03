// Enteros sin signo: con N bits caben los números de 0 a 2^N - 1.
fn main() {
    // 1. El rango de cada tamaño
    println!("u8:  de {} a {}", u8::MIN, u8::MAX);
    println!("u16: de {} a {}", u16::MIN, u16::MAX);
    println!("u32: de {} a {}", u32::MIN, u32::MAX);
    println!("u64: de {} a {}", u64::MIN, u64::MAX);

    // 2. Cada bit vale el doble que el de su derecha
    let n: u8 = 0b1000_0001;
    println!("{:08b} = 128 + 1 = {}", n, n);

    // 3. El máximo es «todos los bits encendidos»
    println!("u8::MAX en binario: {:08b}", u8::MAX);
    println!("u16::MAX en hex:    {:#x}", u16::MAX);
}
