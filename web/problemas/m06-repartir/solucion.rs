fn repartir(v: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut pares = Vec::new();
    let mut impares = Vec::new();
    let mut i = 0;
    for nombre in v {
        if i % 2 == 0 {
            pares.push(nombre);
        } else {
            impares.push(nombre);
        }
        i += 1;
    }
    (pares, impares)
}
