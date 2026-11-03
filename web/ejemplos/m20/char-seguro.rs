// Convierte cada número en char SIN unsafe: si no es válido, usa '?'.
fn a_char(n: u32) -> char {
    char::from_u32(n).unwrap_or('?')
}

fn main() {
    for n in [0x52, 0x75, 0x73, 0x74, 0xD800, 0x1F980, 0x110000] {
        print!("{}", a_char(n));
    }
    println!();
}
