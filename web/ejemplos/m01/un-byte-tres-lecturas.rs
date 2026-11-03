// El mismo byte leído de tres maneras. El byte no cambia; cambia el tipo.
fn main() {
    // 1. Un byte: ocho bits
    let byte: u8 = 0b0100_0001;
    println!("bits:         {:08b}", byte);

    // 2. Leído como número sin signo
    println!("como u8:      {}", byte);

    // 3. Leído como letra
    println!("como letra:   {}", byte as char);

    // 4. Otro byte, con el primer bit encendido, leído con y sin signo
    let otro: u8 = 0b1111_1111;
    println!("bits:         {:08b}", otro);
    println!("como u8:      {}", otro);
    println!("como i8:      {}", otro as i8);
}
