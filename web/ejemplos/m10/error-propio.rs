use std::fmt;

// 1. Un enum con un caso por cada forma de fallar, con los datos útiles.
#[derive(Debug, PartialEq)]
enum ErrorClave {
    Corta { largo: usize, minimo: usize },
    SinNumero,
    Prohibida(String),
}

// 2. Display: el mensaje para personas (lo que verá el usuario).
impl fmt::Display for ErrorClave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorClave::Corta { largo, minimo } => {
                write!(f, "la clave tiene {largo} caracteres y necesita al menos {minimo}")
            }
            ErrorClave::SinNumero => write!(f, "la clave necesita al menos un número"),
            ErrorClave::Prohibida(c) => write!(f, "«{c}» es una clave demasiado común"),
        }
    }
}

// 3. El trait Error: «esto es un error». Con Debug + Display ya basta (todo lo demás tiene valor por defecto).
impl std::error::Error for ErrorClave {}

// 4. La función dice en su firma EXACTAMENTE cómo puede fallar.
fn validar(clave: &str) -> Result<(), ErrorClave> {
    const MINIMO: usize = 8;
    if ["12345678", "contraseña"].contains(&clave) {
        return Err(ErrorClave::Prohibida(clave.to_string()));
    }
    let largo = clave.chars().count();
    if largo < MINIMO {
        return Err(ErrorClave::Corta { largo, minimo: MINIMO });
    }
    if !clave.chars().any(|c| c.is_ascii_digit()) {
        return Err(ErrorClave::SinNumero);
    }
    Ok(())
}

fn main() {
    for clave in ["ñu", "contraseña", "pingüinos", "pingüinos7"] {
        match validar(clave) {
            Ok(()) => println!("{clave}: válida"),
            // 5. Quien llama puede tratar cada caso por separado…
            Err(ErrorClave::SinNumero) => println!("{clave}: añade un número"),
            // …o mostrar el mensaje sin más.
            Err(e) => println!("{clave}: {e}"),
        }
    }
    // 6. Al ser PartialEq, se puede comparar (útil en pruebas).
    println!("{}", validar("abc") == Err(ErrorClave::Corta { largo: 3, minimo: 8 }));
}
