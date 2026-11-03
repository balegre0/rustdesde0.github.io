// El texto se guarda como bytes. Rust usa siempre UTF-8.
fn main() {
    // 1. Cada carácter tiene un número (su punto de código)
    for c in ['A', 'a', 'ñ', '€'] {
        println!("{c} -> número {} -> {:#x}", c as u32, c as u32);
    }

    // 2. Los bytes de un texto: as_bytes()
    for texto in ["Hola", "año", "€"] {
        print!("{texto:<5} -> {} bytes:", texto.len());
        for byte in texto.as_bytes() {
            print!(" {:02x}", byte);
        }
        println!();
    }

    // 3. Los bytes en binario: fíjate en cómo empieza cada uno
    for byte in "ñ".as_bytes() {
        println!("{:08b}", byte);
    }

    // 4. Contar bytes no es contar caracteres
    let s = "año";
    println!("bytes: {}, caracteres: {}", s.len(), s.chars().count());
}
