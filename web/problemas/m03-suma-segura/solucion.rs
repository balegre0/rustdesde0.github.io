fn suma_segura(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}
