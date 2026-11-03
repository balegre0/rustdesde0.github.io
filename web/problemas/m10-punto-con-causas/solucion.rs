use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum ErrorPunto {
    SinComa,
    Coordenada { eje: char, causa: ParseIntError },
}

impl fmt::Display for ErrorPunto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorPunto::SinComa => write!(f, "falta la coma"),
            ErrorPunto::Coordenada { eje, .. } => write!(f, "coordenada {eje} inválida"),
        }
    }
}

impl Error for ErrorPunto {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorPunto::SinComa => None,
            ErrorPunto::Coordenada { causa, .. } => Some(causa),
        }
    }
}

fn leer_punto(texto: &str) -> Result<Punto, ErrorPunto> {
    let Some((a, b)) = texto.split_once(',') else {
        return Err(ErrorPunto::SinComa);
    };
    let x = a.trim().parse().map_err(|causa| ErrorPunto::Coordenada { eje: 'x', causa })?;
    let y = b.trim().parse().map_err(|causa| ErrorPunto::Coordenada { eje: 'y', causa })?;
    Ok(Punto { x, y })
}

fn distancia2(a: &str, b: &str) -> Result<i64, Box<dyn Error>> {
    let p = leer_punto(a)?;
    let q = leer_punto(b)?;
    let dx = q.x as i64 - p.x as i64;
    let dy = q.y as i64 - p.y as i64;
    Ok(dx * dx + dy * dy)
}
