fn main() {
    let obtenido = descripcion_destino();
    let esperado = format!(
        "{}, {} bits, {}",
        std::env::consts::ARCH,
        usize::BITS,
        if cfg!(target_family = "wasm") { "wasm" } else if cfg!(unix) { "unix" } else if cfg!(windows) { "windows" } else { "otra" }
    );
    if obtenido == esperado {
        println!("OK: {obtenido:?}");
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("FALLO: esperaba {esperado:?}, obtuve {obtenido:?}");
    }
}
