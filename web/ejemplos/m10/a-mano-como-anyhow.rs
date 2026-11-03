// La idea de anyhow::Context, escrita a mano: añadir «qué estaba haciendo» a un error.
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ConContexto {
    contexto: String,
    causa: Box<dyn Error>,
}

impl fmt::Display for ConContexto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.contexto)
    }
}

impl Error for ConContexto {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.causa.as_ref())
    }
}

// Un método `contexto` para cualquier Result cuyo error se pueda meter en una caja.
trait Contexto<T> {
    fn contexto(self, texto: &str) -> Result<T, ConContexto>;
}

impl<T, E: Error + 'static> Contexto<T> for Result<T, E> {
    fn contexto(self, texto: &str) -> Result<T, ConContexto> {
        self.map_err(|e| ConContexto { contexto: texto.to_string(), causa: Box::new(e) })
    }
}

fn leer_ancho(texto: &str) -> Result<u32, Box<dyn Error>> {
    let n = texto.parse::<u32>().contexto("leyendo el ancho de la ventana")?;
    Ok(n)
}

fn main() {
    if let Err(e) = leer_ancho("ancho") {
        println!("Error: {e}");
        let mut causa = e.source();
        while let Some(c) = causa {
            println!("Causado por: {c}");
            causa = c.source();
        }
    }
}
