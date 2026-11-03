// Dentro del propio crate, #[non_exhaustive] no restringe nada.
// En OTROS crates obliga a un brazo `_` en match y a usar `..` en patrones de structs.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    Red,
    Disco,
}

#[non_exhaustive]
pub struct Config {
    pub hilos: u32,
}

fn main() {
    let e = Error::Disco;
    let texto = match e {
        Error::Red => "red",
        Error::Disco => "disco", // aquí no hace falta `_`: es el mismo crate
    };
    let c = Config { hilos: 4 };
    let Config { hilos, .. } = c;
    println!("{texto} {hilos}");
}
