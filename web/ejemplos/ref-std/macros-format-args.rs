use std::fmt::Write as _;

// format_args! prepara el texto formateado SIN reservar memoria;
// quien lo recibe decide dónde escribirlo.
fn registrar(args: std::fmt::Arguments) {
    let mut linea = String::from("[registro] ");
    linea.write_fmt(args).unwrap();
    println!("{linea}");
}

fn main() {
    let usuario = "ana";
    registrar(format_args!("entra {usuario} ({} intentos)", 2));
    // Si el texto no tiene argumentos, se puede obtener sin formatear:
    println!("{:?}", format_args!("solo texto").as_str());
}
