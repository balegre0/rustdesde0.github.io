// Edición 2024: no_mangle es un atributo unsafe (dos símbolos iguales serían UB).
#[unsafe(no_mangle)]
pub extern "C" fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", sumar(2, 3));
}
