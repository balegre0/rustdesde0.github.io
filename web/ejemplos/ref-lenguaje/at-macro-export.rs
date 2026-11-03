mod util {
    // #[macro_export] pone la macro en la RAÍZ del crate: se usa como crate::cuadrado!
    #[macro_export]
    macro_rules! cuadrado {
        ($x:expr) => {
            $x * $x
        };
    }
}

// Sin macro_export, una macro_rules! solo se ve DESPUÉS de su definición,
// en su módulo y en los hijos (alcance textual).
macro_rules! doble {
    ($x:expr) => {
        $x * 2
    };
}

fn main() {
    println!("{} {}", crate::cuadrado!(4), doble!(5));
}
