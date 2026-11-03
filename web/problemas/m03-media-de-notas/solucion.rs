fn media(notas: [u8; 5]) -> f64 {
    let mut suma: u32 = 0;
    for nota in notas {
        suma += u32::from(nota);
    }
    suma as f64 / 5.0
}
