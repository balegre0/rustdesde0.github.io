use std::path::{Path, PathBuf};

fn main() {
    // 1. Path es a PathBuf lo que &str es a String
    let p = Path::new("proyectos/juego/src/main.rs");
    println!("archivo:    {:?}", p.file_name());
    println!("sin ext.:   {:?}", p.file_stem());
    println!("extensión:  {:?}", p.extension());
    println!("carpeta:    {:?}", p.parent());

    // 2. Construir rutas sin pegar barras a mano
    let mut q = PathBuf::from("proyectos");
    q.push("juego");
    q.push("Cargo.toml");
    println!("construida: {}", q.display());
    println!("cambiada:   {}", q.with_extension("lock").display());

    // 3. Recorrer los componentes
    let partes: Vec<_> = p.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
    println!("partes:     {partes:?}");

    // 4. Tocar el disco: en el navegador no hay archivos, así que esto falla
    match std::fs::read_to_string(&q) {
        Ok(t) => println!("leí {} bytes", t.len()),
        Err(e) => println!("no pude leer {}: {:?}", q.display(), e.kind()),
    }
}
