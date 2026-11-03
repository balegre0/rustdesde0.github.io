fn fila_csv(campos: &[&str]) -> String {
    let mut fila = campos.join(";");
    fila.push('\n');
    fila
}

fn main() {
    print!("{}", fila_csv(&["nombre", "edad", "ciudad"]));
    print!("{}", fila_csv(&["Íñigo", "40", "Logroño"]));
}
