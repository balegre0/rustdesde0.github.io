// Strict provenance (Rust 1.84): cambiar la dirección SIN perder el permiso.
fn main() {
    let x: u32 = 1234;
    let p: *const u32 = &x;

    // 1. addr(): solo la dirección, como número (sin provenance).
    println!("alineación de u32 = {}; dirección múltiplo de 4: {}", align_of::<u32>(), p.addr() % 4 == 0);

    // 2. map_addr: guardar una «etiqueta» en el bit más bajo (que vale 0 por la alineación).
    let marcado = p.map_addr(|a| a | 1);
    println!("bit de etiqueta = {}", marcado.addr() & 1);

    // 3. Quitar la etiqueta: el puntero conserva la provenance de `x`.
    let limpio = marcado.map_addr(|a| a & !1);
    // SAFETY: limpio tiene la dirección y la provenance originales de p.
    println!("*limpio = {}", unsafe { *limpio });

    // 4. Punteros SIN provenance: valen para comparar o como centinela, nunca para leer.
    let centinela: *const u32 = std::ptr::without_provenance(8);
    println!("centinela.addr() = {}", centinela.addr());
    let colgante = std::ptr::NonNull::<u64>::dangling();
    println!("dangling alineado a 8: {}", colgante.as_ptr().addr() % 8 == 0);
}
