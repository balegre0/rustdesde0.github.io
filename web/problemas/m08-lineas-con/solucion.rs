fn lineas_con<'a>(texto: &'a str, patron: &str) -> Vec<&'a str> {
    let mut v = Vec::new();
    for linea in texto.lines() {
        if linea.contains(patron) {
            v.push(linea);
        }
    }
    v
}
