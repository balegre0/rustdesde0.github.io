// En la edición 2024, los atributos que pueden causar UB van dentro de unsafe(...).
#[no_mangle]
pub extern "C" fn exportada() -> i32 {
    1
}

fn main() {
    println!("{}", exportada());
}
