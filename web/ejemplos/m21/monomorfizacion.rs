use std::any::type_name;
use std::fmt::Debug;

// Una sola función genérica en el código fuente...
fn describe<T: Debug>(x: T) -> String {
    format!("{x:?} es un {}", type_name::<T>())
}

// ...y una con `dyn`: una sola copia para todos los tipos
fn describe_dyn(x: &dyn Debug) -> String {
    format!("{x:?} (sin saber su tipo concreto)")
}

fn main() {
    // 1. Tres usos con tres tipos => el compilador genera TRES copias de `describe`
    println!("{}", describe(7u8));
    println!("{}", describe("hola"));
    println!("{}", describe(vec![1.5, 2.5]));
    // 2. Con dyn: una copia, y la llamada a Debug::fmt pasa por la vtable
    println!("{}", describe_dyn(&7u8));
    println!("{}", describe_dyn(&"hola"));
}
