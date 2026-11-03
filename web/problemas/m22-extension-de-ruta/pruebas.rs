fn main() {
    let rutas = ["src/main.rs", "src/lib.rs", "FOTO.JPG", "notas.txt", "fondo.jpg", "Makefile", ".gitignore", "datos.tar.gz"];
    let obtenido = contar_por_extension(&rutas);
    let esperado: Vec<(&str, usize)> = vec![("(ninguna)", 2), ("gz", 1), ("jpg", 2), ("rs", 2), ("txt", 1)];
    let obtenido_v: Vec<(&str, usize)> = obtenido.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    if obtenido_v == esperado {
        println!("OK: {:?}", obtenido_v);
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("FALLO: esperaba {:?}, obtuve {:?}", esperado, obtenido_v);
    }
}
