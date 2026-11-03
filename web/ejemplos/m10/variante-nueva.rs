#[derive(Debug)]
enum ErrorRed {
    SinConexion,
    TiempoAgotado,
    Rechazado(u16),
}

fn mensaje(e: &ErrorRed) -> &'static str {
    match e {
        ErrorRed::SinConexion => "revisa el cable",
        ErrorRed::TiempoAgotado => "inténtalo más tarde",
    }
}

fn main() {
    println!("{}", mensaje(&ErrorRed::Rechazado(403)));
}
