
fn main() {
    let mut fallos = 0;
    let mut caso = |nombre: &str, ok: bool| {
        if ok { println!("OK: {nombre}"); } else { println!("FALLO: {nombre}"); fallos += 1; }
    };
    let b = [0xAA, 0x01, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12];
    caso("pos 1 (desalineado)", leer_u32_le(&b, 1) == Some(1));
    caso("pos 5", leer_u32_le(&b, 5) == Some(0x1234_5678));
    caso("pos 0", leer_u32_le(&b, 0) == Some(0x0000_01AA));
    caso("justo al final", leer_u32_le(&b, 6).is_none());
    caso("slice corto", leer_u32_le(&[1, 2, 3], 0).is_none());
    caso("pos enorme", leer_u32_le(&b, usize::MAX).is_none());
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
