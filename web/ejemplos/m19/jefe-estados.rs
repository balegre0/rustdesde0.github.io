// El jefe: una macro que genera un enum completo a partir de una lista.
macro_rules! contar {
    () => { 0usize };
    ($cabeza:ident $($resto:ident)*) => { 1usize + contar!($($resto)*) };
}

macro_rules! estados {
    ($(#[$meta:meta])* $vis:vis enum $nombre:ident { $($var:ident),+ $(,)? }) => {
        // 1. El enum, con los atributos que nos pasen
        $(#[$meta])*
        $vis enum $nombre { $($var),+ }

        impl $nombre {
            // 2. Cuántas variantes hay (calculado en compilación)
            pub const N: usize = contar!($($var)*);
            // 3. Todas las variantes, en orden
            pub const TODOS: [$nombre; contar!($($var)*)] = [$($nombre::$var),+];
            // 4. El nombre de cada variante como texto
            pub fn nombre(&self) -> &'static str {
                match self { $($nombre::$var => stringify!($var)),+ }
            }
        }
    };
}

estados! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Semaforo { Verde, Ambar, Rojo }
}

fn main() {
    println!("hay {} estados", Semaforo::N);
    for s in Semaforo::TODOS {
        println!("{:?} -> {}", s, s.nombre());
    }
    println!("{}", Semaforo::Rojo == Semaforo::TODOS[2]);
}
