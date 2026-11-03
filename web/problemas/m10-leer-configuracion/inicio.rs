#[derive(Debug, PartialEq)]
struct Config {
    nombre: String,
    puerto: u16,
    depurar: bool,
}

#[derive(Debug, PartialEq)]
enum ErrorConfig {
    Formato(usize),
    ClaveDesconocida(usize, String),
    Valor(usize, String),
    Falta(&'static str),
}

fn leer(texto: &str) -> Result<Config, ErrorConfig> {
    todo!()
}
