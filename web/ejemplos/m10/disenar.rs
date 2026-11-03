use std::fmt;

// 1. Error de una «biblioteca» de reservas: enum concreto, con datos útiles.
#[derive(Debug, PartialEq)]
enum ErrorReserva {
    SinPlazas { pedidas: u32, libres: u32 },
    FechaPasada,
}

impl fmt::Display for ErrorReserva {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorReserva::SinPlazas { pedidas, libres } => write!(f, "pediste {pedidas} plazas y solo quedan {libres}"),
            ErrorReserva::FechaPasada => write!(f, "esa fecha ya pasó"),
        }
    }
}

impl std::error::Error for ErrorReserva {}

/// Reserva plazas.
///
/// # Errors
/// - `SinPlazas` si no quedan suficientes.
/// - `FechaPasada` si `dia` es anterior a `hoy`.
///
/// # Panics
/// Si `pedidas` es 0: pedir cero plazas es un fallo de quien llama.
fn reservar(libres: &mut u32, pedidas: u32, dia: u32, hoy: u32) -> Result<u32, ErrorReserva> {
    assert!(pedidas > 0, "reservar 0 plazas no tiene sentido");
    if dia < hoy {
        return Err(ErrorReserva::FechaPasada);
    }
    if pedidas > *libres {
        return Err(ErrorReserva::SinPlazas { pedidas, libres: *libres });
    }
    *libres -= pedidas;
    Ok(*libres)
}

fn main() {
    let mut libres = 5;
    // 2. Quien llama decide: reintentar, avisar, o tratar cada caso distinto.
    for (pedidas, dia) in [(2, 10), (4, 10), (1, 3), (3, 12)] {
        match reservar(&mut libres, pedidas, dia, 5) {
            Ok(quedan) => println!("reservadas {pedidas}; quedan {quedan}"),
            Err(ErrorReserva::SinPlazas { libres, .. }) if libres > 0 => {
                println!("no caben {pedidas}; ¿quieres las {libres} que quedan?")
            }
            Err(e) => println!("no se pudo: {e}"),
        }
    }
}
