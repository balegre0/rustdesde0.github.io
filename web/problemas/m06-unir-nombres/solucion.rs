fn unir(v: Vec<String>) -> String {
    let mut resultado = String::new();
    let mut primero = true;
    for nombre in v {
        if !primero {
            resultado.push_str(", ");
        }
        primero = false;
        resultado.push_str(&nombre);
    }
    resultado
}
