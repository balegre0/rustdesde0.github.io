// En la edición 2024, `no_mangle` es un atributo inseguro:
// hay que escribirlo `#[unsafe(no_mangle)]`.
#[no_mangle]
pub extern "C" fn exportada() -> i32 { 1 }

fn main() {}
