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
    let (mut nombre, mut puerto, mut depurar) = (None, None, None);
    for (i, linea) in texto.lines().enumerate() {
        let n = i + 1;
        let linea = linea.trim();
        if linea.is_empty() || linea.starts_with('#') {
            continue;
        }
        let (clave, valor) = linea.split_once('=').ok_or(ErrorConfig::Formato(n))?;
        let (clave, valor) = (clave.trim(), valor.trim());
        let error_valor = || ErrorConfig::Valor(n, clave.to_string());
        match clave {
            "nombre" => nombre = Some(valor.to_string()),
            "puerto" => puerto = Some(valor.parse::<u16>().map_err(|_| error_valor())?),
            "depurar" => depurar = Some(valor.parse::<bool>().map_err(|_| error_valor())?),
            otra => return Err(ErrorConfig::ClaveDesconocida(n, otra.to_string())),
        }
    }
    Ok(Config {
        nombre: nombre.ok_or(ErrorConfig::Falta("nombre"))?,
        puerto: puerto.ok_or(ErrorConfig::Falta("puerto"))?,
        depurar: depurar.ok_or(ErrorConfig::Falta("depurar"))?,
    })
}
