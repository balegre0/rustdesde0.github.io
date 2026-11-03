use std::path::{Path, PathBuf};

fn configuracion(base: &Path) -> PathBuf {
    // Devolvemos el PathBuf con propiedad: quien llama se queda con él
    base.join("config.toml")
}

fn main() {
    println!("{}", configuracion(Path::new("/etc/app")).display());
}
