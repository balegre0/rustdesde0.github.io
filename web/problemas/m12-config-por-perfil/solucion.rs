fn descripcion_destino() -> String {
    let familia = if cfg!(target_family = "wasm") {
        "wasm"
    } else if cfg!(unix) {
        "unix"
    } else if cfg!(windows) {
        "windows"
    } else {
        "otra"
    };
    format!("{}, {} bits, {}", std::env::consts::ARCH, usize::BITS, familia)
}
