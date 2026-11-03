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
        todo!()
    }
}

impl std::error::Error for ErrorHora {}

fn parsear_hora(texto: &str) -> Result<Hora, ErrorHora> {
    todo!()
}
