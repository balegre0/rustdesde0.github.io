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
    todo!()
}
