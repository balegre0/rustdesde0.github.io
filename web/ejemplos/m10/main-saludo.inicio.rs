// Haz que main devuelva Result<(), ParseIntError> y usa `?` en vez de unwrap.
// Con la entrada "3" debe imprimir hola tres veces.

use std::num::ParseIntError;

fn main() {
    let veces: u32 = "3".parse().unwrap();
    for _ in 0..veces {
        println!("hola");
    }
}
