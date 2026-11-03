use std::path::Path;

fn main() {
    // 1. Una ruta relativa: empieza en la carpeta donde estás.
    let ruta = Path::new("proyectos/hola/src/main.rs");

    // 2. Rust sabe partirla en piezas.
    println!("archivo:   {:?}", ruta.file_name());
    println!("nombre:    {:?}", ruta.file_stem());
    println!("extensión: {:?}", ruta.extension());
    println!("carpeta:   {:?}", ruta.parent());

    // 3. Cada carpeta del camino, una a una.
    for pieza in ruta.iter() {
        println!("  -> {:?}", pieza);
    }
}
