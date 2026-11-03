// Atributos que actúan sobre los símbolos que ve el enlazador (inseguros en la edición 2024).
#[unsafe(no_mangle)]
pub extern "C" fn rdc_sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(export_name = "rdc_restar_v2")]
pub extern "C" fn restar(a: i32, b: i32) -> i32 {
    a - b
}

// #[used] obliga a conservar el static aunque nada lo lea.
#[used]
static VERSION: [u8; 4] = *b"1.0\0";

fn main() {
    println!("{} {}", rdc_sumar(2, 3), restar(5, 1));
}
