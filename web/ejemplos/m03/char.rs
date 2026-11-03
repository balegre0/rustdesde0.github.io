use std::mem::size_of;

fn main() {
    // 1. bool: solo true o false
    let llueve = false;
    let hace_sol = !llueve;
    println!("llueve: {llueve}, sol: {hace_sol}");
    println!("y: {}, o: {}", llueve && hace_sol, llueve || hace_sol);

    // 2. char: comillas simples, cualquier carácter Unicode
    let letras = ['a', 'ñ', 'Ω', '€', '🦀'];
    for c in letras {
        println!("{c} -> U+{:04X}, bytes en UTF-8: {}", c as u32, c.len_utf8());
    }

    // 3. Pero un char siempre ocupa 4 bytes en memoria
    println!("bool ocupa {} byte, char ocupa {} bytes", size_of::<bool>(), size_of::<char>());
}
