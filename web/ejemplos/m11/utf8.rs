fn main() {
    let palabras = ["sol", "año", "pingüino", "€", "👋"];

    // 1. len() cuenta BYTES; chars().count() cuenta caracteres (char).
    for p in palabras {
        println!("{p:>9}: {} bytes, {} chars", p.len(), p.chars().count());
    }

    // 2. Los bytes de «año»: la ñ ocupa 2 (0xC3 0xB1).
    let anio = "año";
    print!("bytes de «año»:");
    for b in anio.bytes() {
        print!(" {b:#04x}");
    }
    println!();

    // 3. chars(): recorrer carácter a carácter.
    for c in "Ñu".chars() {
        println!("'{c}' es U+{:04X} y ocupa {} bytes en UTF-8", c as u32, c.len_utf8());
    }

    // 4. char_indices(): cada carácter con el byte donde EMPIEZA.
    for (i, c) in "añil".char_indices() {
        print!("{i}:{c} ");
    }
    println!();

    // 5. El carácter número n: con chars().nth(n) (recorre desde el principio).
    println!("carácter nº 5 (contando desde 0) de «pingüino»: {:?}", "pingüino".chars().nth(5));

    // 6. Un char ocupa siempre 4 bytes en memoria (aunque en UTF-8 ocupe de 1 a 4).
    println!("size_of::<char>() = {}", std::mem::size_of::<char>());
}
