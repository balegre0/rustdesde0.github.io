#[derive(Debug, PartialEq)]
enum Lado {
    Izquierda,
    Derecha,
}

#[derive(Debug, PartialEq)]
enum Comando {
    Avanzar(u32),
    Girar(Lado),
    Parar,
}

fn parsear(linea: &str) -> Option<Comando> {
    let palabras: Vec<&str> = linea.split_whitespace().collect();
    match palabras.as_slice() {
        ["para"] => Some(Comando::Parar),
        ["gira", "izquierda"] => Some(Comando::Girar(Lado::Izquierda)),
        ["gira", "derecha"] => Some(Comando::Girar(Lado::Derecha)),
        ["avanza", n] => {
            let Ok(pasos) = n.parse::<u32>() else {
                return None;
            };
            Some(Comando::Avanzar(pasos))
        }
        _ => None,
    }
}
