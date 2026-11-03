const LETRAS: &[u8; 23] = b"TRWAGMYFPDXBNJZSQVHLCKE";

fn letra(numero: u32) -> char {
    LETRAS[(numero % 23) as usize] as char
}

fn validar(dni: &str) -> bool {
    let b = dni.as_bytes();
    if b.len() != 9 || !b[..8].iter().all(u8::is_ascii_digit) {
        return false;
    }
    let numero: u32 = dni[..8].parse().unwrap_or(0);
    b[8] as char == letra(numero)
}
