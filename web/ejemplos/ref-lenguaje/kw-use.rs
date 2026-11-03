// 1. Importar un elemento.
use std::collections::HashMap;
// 2. Varios a la vez, con llaves anidadas.
use std::fmt::{self, Display};
// 3. Renombrar.
use std::io::Result as IoResult;

mod util {
    pub fn doble(x: i32) -> i32 { x * 2 }
    pub fn triple(x: i32) -> i32 { x * 3 }
}
// 4. Glob: todo lo público de `util`.
use util::*;

// 5. Reexportar: `pub use` hace visible el elemento desde aquí.
pub use util::doble as duplicar;

struct Nota(u8);
impl Display for Nota {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "nota {}", self.0) }
}

fn nada() -> IoResult<()> { Ok(()) }

fn main() {
    let mut m = HashMap::new();
    m.insert(1, "uno");
    println!("{:?}", m.get(&1));
    println!("{} {}", doble(2), triple(2));
    println!("{}", duplicar(10));
    println!("{}", Nota(9));
    println!("{:?}", nada());
}
