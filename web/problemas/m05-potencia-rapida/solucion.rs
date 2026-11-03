fn potencia(base: u64, exp: u32) -> u64 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        let mitad = potencia(base, exp / 2);
        mitad * mitad
    } else {
        base * potencia(base, exp - 1)
    }
}
