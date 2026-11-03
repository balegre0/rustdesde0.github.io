use std::fmt;

#[derive(Debug, PartialEq)]
struct Hora {
    h: u8,
    m: u8,
}

#[derive(Debug, PartialEq)]
enum ErrorHora {
    Formato,
    HoraInvalida(u8),
    MinutoInvalido(u8),
}

impl fmt::Display for ErrorHora {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorHora::Formato => write!(f, "usa el formato HH:MM"),
            ErrorHora::HoraInvalida(h) => write!(f, "la hora {h} no existe"),
            ErrorHora::MinutoInvalido(m) => write!(f, "el minuto {m} no existe"),
        }
    }
}

impl std::error::Error for ErrorHora {}

fn parsear_hora(texto: &str) -> Result<Hora, ErrorHora> {
    let Some((a, b)) = texto.split_once(':') else {
        return Err(ErrorHora::Formato);
    };
    let h: u8 = a.parse().map_err(|_| ErrorHora::Formato)?;
    let m: u8 = b.parse().map_err(|_| ErrorHora::Formato)?;
    if h > 23 {
        return Err(ErrorHora::HoraInvalida(h));
    }
    if m > 59 {
        return Err(ErrorHora::MinutoInvalido(m));
    }
    Ok(Hora { h, m })
}
