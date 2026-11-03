use std::path::{Path, PathBuf};

fn configuracion(base: &Path) -> PathBuf {
    // join devuelve un PathBuf nuevo; esta línea devuelve un &Path prestado de un temporal
    base.join("config.toml").as_path()
}

fn main() {
    println!("{}", configuracion(Path::new("/etc/app")).display());
}
