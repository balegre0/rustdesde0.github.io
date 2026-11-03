// Lo que la biblioteca thiserror escribiría por ti, escrito a mano.
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorDescarga {
    // con thiserror:  #[error("la URL «{0}» no es válida")]
    UrlInvalida(String),
    // con thiserror:  #[error("puerto inválido")]  +  #[from] en el campo
    Puerto(ParseIntError),
}

// 1. Display: una línea por variante (lo que generan los #[error("…")]).
impl fmt::Display for ErrorDescarga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorDescarga::UrlInvalida(u) => write!(f, "la URL «{u}» no es válida"),
            ErrorDescarga::Puerto(_) => write!(f, "puerto inválido"),
        }
    }
}

// 2. Error con source(): el error envuelto es la causa (lo que genera #[from] o #[source]).
impl Error for ErrorDescarga {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorDescarga::UrlInvalida(_) => None,
            ErrorDescarga::Puerto(e) => Some(e),
        }
    }
}

// 3. From para que `?` convierta (lo que genera #[from]).
impl From<ParseIntError> for ErrorDescarga {
    fn from(e: ParseIntError) -> Self {
        ErrorDescarga::Puerto(e)
    }
}

fn puerto(url: &str) -> Result<u16, ErrorDescarga> {
    let Some((_, p)) = url.rsplit_once(':') else {
        return Err(ErrorDescarga::UrlInvalida(url.to_string()));
    };
    Ok(p.parse::<u16>()?)
}

fn main() {
    for url in ["servidor:8080", "servidor", "servidor:99999"] {
        match puerto(url) {
            Ok(p) => println!("{url}: puerto {p}"),
            Err(e) => {
                print!("{url}: {e}");
                if let Some(causa) = e.source() {
                    print!(" (causa: {causa})");
                }
                println!();
            }
        }
    }
}
