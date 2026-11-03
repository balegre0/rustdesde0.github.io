fn vocales(texto: &str) -> usize {
    let mut n = 0;
    for c in texto.chars() {
        if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'á' | 'é' | 'í' | 'ó' | 'ú') {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("{}", vocales("murciélago"));
    println!("{}", vocales("ñandú"));
}
