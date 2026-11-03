// JEFE DEL MUNDO 10: el lector de configuración.
// Completa `leer_linea` y `leer_config`. Formato: una línea por opción, «clave = valor».
//  - Líneas vacías o que empiezan por '#': se ignoran.
//  - Sin '=': ErrorConfig::SinIgual { linea }.
//  - Clave desconocida (solo valen "ancho", "alto", "titulo"): ErrorConfig::ClaveDesconocida { linea, clave }.
//  - "ancho" y "alto" deben ser u32: si no, ErrorConfig::Numero { linea, causa } (usa map_err).
//  - Al final, si falta "ancho" o "alto": ErrorConfig::Falta("ancho") / Falta("alto").
// Los números de línea empiezan en 1.

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
    // Pista: let Some((clave, valor)) = linea.split_once('=') else { … };
    //        luego clave.trim() y valor.trim(), y un match sobre la clave.
    todo!()
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
    // Pista: let ancho = cfg.ancho.ok_or(ErrorConfig::Falta("ancho"))?;
    todo!()
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
