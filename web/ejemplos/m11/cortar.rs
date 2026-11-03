fn main() {
    let texto = "España,Perú,México";

    // 1. split: trocear por un separador. Cada trozo es un &str prestado de `texto`.
    for pais in texto.split(',') {
        print!("[{pais}] ");
    }
    println!();

    // 2. split_whitespace: por espacios (uno o varios). trim: quitar espacios de los bordes.
    let sucio = "  uno   dos\ttres \n";
    println!("{:?} → {:?}", sucio.trim(), sucio.split_whitespace().collect::<Vec<_>>());

    // 3. lines: por saltos de línea.
    let poema = "verde que te quiero verde\nverde viento\nverdes ramas";
    println!("líneas: {}", poema.lines().count());

    // 4. Buscar: contains, starts_with, ends_with, find (posición en BYTES).
    println!("{} {} {:?}", texto.contains("Perú"), texto.starts_with("Esp"), texto.find("Perú"));

    // 5. split_once: partir en dos por la primera aparición.
    println!("{:?}", "clave=valor=extra".split_once('='));

    // 6. Cortar por bytes SOLO en fronteras de carácter. find da una frontera válida.
    if let Some(i) = texto.find(',') {
        println!("primer país: {}", &texto[..i]);
    }
    println!("¿el byte 5 es frontera en «España»? {}", "España".is_char_boundary(5));
    println!("¿el byte 4 es frontera en «España»? {}", "España".is_char_boundary(4));

    // 7. get(a..b): como [a..b] pero devuelve Option en vez de pánico.
    println!("{:?} {:?}", "España".get(0..4), "España".get(0..5));

    // 8. Mayúsculas y minúsculas con Unicode.
    println!("{} {}", "ñandú".to_uppercase(), "ÁRBOL".to_lowercase());
}
