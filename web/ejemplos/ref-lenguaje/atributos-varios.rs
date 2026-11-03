// Atributo interno del crate: se aplica a todo el archivo.
#![allow(unused_variables)]

use std::fmt;

// derive: implementaciones automáticas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Version { mayor: u32, menor: u32 }

// must_use: avisa si alguien ignora el resultado.
#[must_use = "la suma no modifica nada; usa el resultado"]
fn sumar(a: u32, b: u32) -> u32 { a + b }

// deprecated: avisa a quien lo use.
#[deprecated(since = "2.0.0", note = "usa `sumar`")]
#[allow(dead_code)]
fn sumar_viejo(a: u32, b: u32) -> u32 { a + b }

// cfg: compilación condicional.
#[cfg(target_pointer_width = "64")]
const BITS: &str = "64 bits";
#[cfg(not(target_pointer_width = "64"))]
const BITS: &str = "no 64 bits";

// inline y track_caller.
#[inline]
#[track_caller]
fn donde() -> String {
    let l = std::panic::Location::caller();
    format!("línea {}", l.line())
}

// non_exhaustive: fuera de este crate obliga a usar `_` o `..`.
#[non_exhaustive]
#[derive(Debug)]
pub enum Modo { Rapido, Lento }

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.mayor, self.menor)
    }
}

fn main() {
    let v = Version { mayor: 1, menor: 99 };
    println!("{v} {:?} {}", Version::default(), v > Version::default());
    println!("{}", sumar(2, 2));
    println!("{BITS}");
    println!("{}", donde());
    // expect: como allow, pero avisa si el lint NO salta.
    #[expect(unused_variables)]
    let sin_usar = 1;
    println!("{}", cfg!(debug_assertions));
    println!("{:?}", Modo::Lento);
    let _ = Modo::Rapido;
}
