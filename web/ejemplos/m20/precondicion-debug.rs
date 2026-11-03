// En modo debug, la biblioteca estándar comprueba algunas precondiciones
// de funciones unsafe ANTES de que ocurra el UB, y aborta el programa.
fn main() {
    let mut v = [1u8, 2, 3, 4];
    let p = v.as_mut_ptr();
    println!("antes de copiar");
    // ¡MAL! origen [0..3) y destino [1..4) se solapan: copy_nonoverlapping
    // exige que NO se solapen.
    unsafe { std::ptr::copy_nonoverlapping(p, p.add(1), 3) };
    println!("después: {v:?}");
}
