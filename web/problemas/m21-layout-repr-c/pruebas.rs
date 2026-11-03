
fn main() {
    let mut fallos = 0;
    let mut caso = |nombre: &str, obtenido: (Vec<usize>, usize, usize), esperado: (Vec<usize>, usize, usize)| {
        if obtenido == esperado { println!("OK: {nombre}"); } else { fallos += 1; println!("FALLO: {nombre}: esperaba {esperado:?}, obtuve {obtenido:?}"); }
    };
    caso("u8, u32, u16", layout_c(&[(1, 1), (4, 4), (2, 2)]), (vec![0, 4, 8], 12, 4));
    caso("u32, u16, u8", layout_c(&[(4, 4), (2, 2), (1, 1)]), (vec![0, 4, 6], 8, 4));
    caso("vacío", layout_c(&[]), (vec![], 0, 1));
    caso("u8, u64", layout_c(&[(1, 1), (8, 8)]), (vec![0, 8], 16, 8));
    caso("tres u8", layout_c(&[(1, 1), (1, 1), (1, 1)]), (vec![0, 1, 2], 3, 1));
    caso("u16 y [u8; 3]", layout_c(&[(2, 2), (3, 1)]), (vec![0, 2], 6, 2));
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
