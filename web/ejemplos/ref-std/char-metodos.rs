fn main() {
    for c in ['a', 'Ñ', '7', ' ', 'é', '€', '\n'] {
        println!(
            "{:?}: alfabética {} · numérica {} · espacio {} · mayúscula {} · U+{:04X} · {} byte(s)",
            c, c.is_alphabetic(), c.is_numeric(), c.is_whitespace(), c.is_uppercase(), c as u32, c.len_utf8()
        );
    }
    println!("{}", 'ñ'.to_uppercase());
    println!("{:?}", '7'.to_digit(10));
    println!("{:?}", char::from_digit(9, 10));
    println!("{:?}", char::from_u32(0x1F980));
    println!("{}", ('a'..='e').collect::<String>());
}
