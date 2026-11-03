// Fragmentos: expr, ident, ty, tt; repeticiones; recursión; $crate.
macro_rules! campo_getter {
    ($nombre:ident : $t:ty) => {
        fn $nombre(&self) -> $t { self.$nombre }
    };
}

struct Punto { x: i32, y: i32 }
impl Punto {
    campo_getter!(x: i32);
    campo_getter!(y: i32);
}

// Repetición con separador y recursión sobre tokens (tt).
macro_rules! contar {
    () => { 0usize };
    ($cabeza:tt $($resto:tt)*) => { 1usize + contar!($($resto)*) };
}

// Varias reglas: la primera que encaja gana.
macro_rules! que_es {
    ($l:literal) => { "un literal" };
    ($i:ident) => { "un identificador" };
    ($e:expr) => { "una expresión" };
}

// Mapa con pares clave => valor.
macro_rules! mapa {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::BTreeMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

fn main() {
    let p = Punto { x: 3, y: 4 };
    println!("{} {}", p.x(), p.y());
    println!("{}", contar!(a b c d));
    println!("{} / {} / {}", que_es!(5), que_es!(hola), que_es!(1 + 1));
    println!("{:?}", mapa! { "a" => 1, "b" => 2, });
    // Higiene: la `x` de la macro no es la `x` de main.
    macro_rules! poner_x { ($v:expr) => { let x = $v; let _ = x; }; }
    let x = 1;
    poner_x!(99);
    println!("x sigue valiendo {x}");
    println!("{}", stringify!(1 + 2 * 3));
}
