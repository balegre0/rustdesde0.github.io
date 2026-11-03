fn leer_u32_le(bytes: &[u8], pos: usize) -> Option<u32> {
    let fin = pos.checked_add(4)?;
    if fin > bytes.len() {
        return None;
    }
    // SAFETY: pos + 4 <= len, así que los 4 bytes están dentro del slice;
    // read_unaligned no exige alineación y cualquier patrón de bits es un u32 válido.
    let crudo = unsafe { bytes.as_ptr().add(pos).cast::<u32>().read_unaligned() };
    Some(u32::from_le(crudo))
}
