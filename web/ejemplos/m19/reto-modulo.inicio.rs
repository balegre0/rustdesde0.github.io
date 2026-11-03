// Haz que `area!` se pueda usar desde el módulo `figuras` y desde main
// importándola por ruta (sin moverla de `macros`).
mod macros {
    macro_rules! area {
        ($b:expr, $h:expr) => { $b * $h };
    }
}

mod figuras {
    // use ...;
    pub fn rectangulo() -> u32 { 0 /* area!(3, 4) */ }
}

fn main() {
    println!("{}", figuras::rectangulo());
    // println!("{}", crate::macros::area!(5, 2));
}
