fn parsear_hora(texto: &str) -> Option<(u8, u8)> {
    let b = texto.as_bytes();
    if b.len() != 5 || b[2] != b':' {
        return None;
    }
    let digitos = [b[0], b[1], b[3], b[4]];
    if !digitos.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let h = (b[0] - b'0') * 10 + (b[1] - b'0');
    let m = (b[3] - b'0') * 10 + (b[4] - b'0');
    if h > 23 || m > 59 {
        return None;
    }
    Some((h, m))
}
