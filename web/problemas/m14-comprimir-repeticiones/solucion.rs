fn comprimir(texto: &str) -> Vec<(char, usize)> {
    let mut resultado = Vec::new();
    let mut it = texto.chars().peekable();
    while let Some(c) = it.next() {
        let mut cuenta = 1;
        while it.next_if_eq(&c).is_some() {
            cuenta += 1;
        }
        resultado.push((c, cuenta));
    }
    resultado
}
