fn subir_volumen(volumen: u8, pulsaciones: u8) -> u8 {
    volumen.saturating_add(pulsaciones.saturating_mul(10))
}
