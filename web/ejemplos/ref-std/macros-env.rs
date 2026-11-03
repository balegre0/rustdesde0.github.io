fn main() {
    // option_env!: Some si la variable existía AL COMPILAR, None si no.
    let ruta: Option<&str> = option_env!("RDC_VARIABLE_QUE_NO_EXISTE");
    println!("{:?}", ruta);
    // env!("...") con una variable que no exista sería un error de compilación.
    // Con Cargo siempre existen, por ejemplo, CARGO_PKG_NAME y CARGO_PKG_VERSION.
    let nombre = option_env!("CARGO_PKG_NAME").unwrap_or("(sin Cargo)");
    println!("paquete: {nombre}");
}
