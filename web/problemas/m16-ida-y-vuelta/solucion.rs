fn comprimir(s: &str) -> String {
    let mut salida = String::new();
    let mut chars = s.chars();
    let Some(mut actual) = chars.next() else {
        return salida;
    };
    let mut cuenta = 1;
    for c in chars {
        if c == actual {
            cuenta += 1;
        } else {
            salida.push_str(&cuenta.to_string());
            salida.push(actual);
            actual = c;
            cuenta = 1;
        }
    }
    salida.push_str(&cuenta.to_string());
    salida.push(actual);
    salida
}

fn descomprimir(s: &str) -> Option<String> {
    let mut salida = String::new();
    let mut numero: Option<usize> = None;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            numero = Some(numero.unwrap_or(0) * 10 + d as usize);
        } else {
            let n = numero.take()?;
            for _ in 0..n {
                salida.push(c);
            }
        }
    }
    if numero.is_some() {
        return None;
    }
    Some(salida)
}
