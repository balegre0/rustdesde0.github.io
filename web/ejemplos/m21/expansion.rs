// La expansión de macros trabaja con ÁRBOLES de tokens, no con texto.
macro_rules! cuadrado {
    ($e:expr) => { $e * $e };
}

// Higiene: la `x` de dentro de la macro no es la `x` de fuera.
macro_rules! doble_de {
    ($e:expr) => {{
        let x = 2;
        x * $e
    }};
}

fn main() {
    // 1. $e es UNA expresión ya agrupada: (2 + 1) * (2 + 1), no 2 + 1 * 2 + 1
    println!("cuadrado!(2 + 1) = {}", cuadrado!(2 + 1));
    // 2. La x de main vale 10; la de la macro vale 2 y no la pisa
    let x = 10;
    println!("doble_de!(x) = {}", doble_de!(x));
    println!("x sigue valiendo {x}");
}
