#[derive(Debug, PartialEq, Clone, Copy)]
enum Estado {
    Parado { planta: u8 },
    Subiendo { desde: u8, hasta: u8 },
    Bajando { desde: u8, hasta: u8 },
}

#[derive(Debug, Clone, Copy)]
enum Evento {
    Llamar(u8),
    Llegar,
    Emergencia,
}

fn transicion(estado: Estado, evento: Evento) -> Estado {
    todo!()
}
