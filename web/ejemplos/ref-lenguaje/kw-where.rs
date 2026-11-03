use std::fmt::Debug;

// 1. `where` mueve los límites fuera de la lista de genéricos: más legible.
fn comparar<T, U>(a: T, b: U) -> String
where
    T: Debug + PartialOrd<U>,
    U: Debug,
{
    if a < b { format!("{a:?} < {b:?}") } else { format!("{a:?} >= {b:?}") }
}

// 2. `where` permite límites sobre tipos que no son parámetros, como `Vec<T>`.
fn imprimir_todo<T>(v: Vec<T>)
where
    Vec<T>: Debug,
{
    println!("{v:?}");
}

fn main() {
    println!("{}", comparar(3, 5));
    imprimir_todo(vec!['a', 'b']);
}
