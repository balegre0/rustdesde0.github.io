// JEFE DEL MUNDO 10: el lector de configuración (solución).

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorConfig {
    SinIgual { linea: usize },
    ClaveDesconocida { linea: usize, clave: String },
    Numero { linea: usize, causa: ParseIntError },
    Falta(&'static str),
}

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorConfig::SinIgual { linea } => write!(f, "línea {linea}: falta el «=»"),
            ErrorConfig::ClaveDesconocida { linea, clave } => write!(f, "línea {linea}: no conozco la opción «{clave}»"),
            ErrorConfig::Numero { linea, .. } => write!(f, "línea {linea}: número inválido"),
            ErrorConfig::Falta(clave) => write!(f, "falta la opción obligatoria «{clave}»"),
        }
    }
}

impl std::error::Error for ErrorConfig {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorConfig::Numero { causa, .. } => Some(causa),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct Config {
    ancho: Option<u32>,
    alto: Option<u32>,
    titulo: Option<String>,
}

fn leer_linea(cfg: &mut Config, n: usize, linea: &str) -> Result<(), ErrorConfig> {
    let Some((clave, valor)) = linea.split_once('=') else {
        return Err(ErrorConfig::SinIgual { linea: n });
    };
    let (clave, valor) = (clave.trim(), valor.trim());
    let numero = |v: &str| v.parse::<u32>().map_err(|causa| ErrorConfig::Numero { linea: n, causa });
    match clave {
        "ancho" => cfg.ancho = Some(numero(valor)?),
        "alto" => cfg.alto = Some(numero(valor)?),
        "titulo" => cfg.titulo = Some(valor.to_string()),
        otra => return Err(ErrorConfig::ClaveDesconocida { linea: n, clave: otra.to_string() }),
    }
    Ok(())
}

fn leer_config(texto: &str) -> Result<(u32, u32, String), ErrorConfig> {
    let mut cfg = Config::default();
    for (i, linea) in texto.lines().enumerate() {
        let linea = linea.trim();
        if linea.is_empty() || linea.starts_with('#') {
            continue;
        }
        leer_linea(&mut cfg, i + 1, linea)?;
    }
    let ancho = cfg.ancho.ok_or(ErrorConfig::Falta("ancho"))?;
    let alto = cfg.alto.ok_or(ErrorConfig::Falta("alto"))?;
    let titulo = cfg.titulo.unwrap_or_else(|| String::from("sin título"));
    Ok((ancho, alto, titulo))
}

fn main() {
    let pruebas = [
        "# mi ventana\nancho = 800\nalto = 600\ntitulo = Rust desde cero",
        "ancho = 800\nalto 600",
        "ancho = 800\nprofundidad = 3",
        "ancho = ochocientos",
        "ancho = 800",
    ];
    for texto in pruebas {
        match leer_config(texto) {
            Ok((an, al, t)) => println!("OK: {an}×{al} «{t}»"),
            Err(e) => {
                print!("Error: {e}");
                if let Some(c) = std::error::Error::source(&e) {
                    print!(" — causa: {c}");
                }
                println!();
            }
        }
    }
}
