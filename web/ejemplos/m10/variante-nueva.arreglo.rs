#[derive(Debug)]
enum ErrorRed {
    SinConexion,
    TiempoAgotado,
    Rechazado(u16),
}

fn mensaje(e: &ErrorRed) -> String {
    match e {
        ErrorRed::SinConexion => String::from("revisa el cable"),
        ErrorRed::TiempoAgotado => String::from("inténtalo más tarde"),
        ErrorRed::Rechazado(codigo) => format!("el servidor dijo que no ({codigo})"),
    }
}

fn main() {
    for e in [ErrorRed::SinConexion, ErrorRed::TiempoAgotado, ErrorRed::Rechazado(403)] {
        println!("{:?}: {}", e, mensaje(&e));
    }
}
