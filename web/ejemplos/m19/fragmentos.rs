// 1. ident: un nombre; ty: un tipo; expr: una expresión
macro_rules! constante {
    ($nombre:ident : $tipo:ty = $valor:expr) => {
        fn $nombre() -> $tipo { $valor }
    };
}

// 2. literal: solo literales; block: un bloque { ... }
macro_rules! repite {
    ($veces:literal, $cuerpo:block) => {
        for _ in 0..$veces $cuerpo
    };
}

constante!(respuesta: u32 = 6 * 7);
constante!(saludo: &'static str = "buenas");

fn main() {
    println!("{} {}", respuesta(), saludo());
    repite!(3, { print!("ñ "); });
    println!();
}
