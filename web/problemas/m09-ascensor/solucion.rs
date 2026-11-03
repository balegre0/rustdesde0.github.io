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
    match (estado, evento) {
        (Estado::Subiendo { desde, .. } | Estado::Bajando { desde, .. }, Evento::Emergencia) => {
            Estado::Parado { planta: desde }
        }
        (Estado::Subiendo { hasta, .. } | Estado::Bajando { hasta, .. }, Evento::Llegar) => {
            Estado::Parado { planta: hasta }
        }
        (Estado::Parado { planta }, Evento::Llamar(p)) if p <= 10 && p > planta => {
            Estado::Subiendo { desde: planta, hasta: p }
        }
        (Estado::Parado { planta }, Evento::Llamar(p)) if p < planta => {
            Estado::Bajando { desde: planta, hasta: p }
        }
        (e, _) => e,
    }
}
