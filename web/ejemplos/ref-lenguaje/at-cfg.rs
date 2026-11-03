// cfg quita el elemento si no se cumple; cfg_attr añade atributos si se cumple.
#[cfg(target_family = "wasm")]
fn plataforma() -> &'static str {
    "wasm"
}

#[cfg(not(target_family = "wasm"))]
fn plataforma() -> &'static str {
    "nativa"
}

#[cfg_attr(target_pointer_width = "32", derive(Debug))]
struct Dato(u8);

fn main() {
    println!("{}", plataforma());
    let d = Dato(7);
    #[cfg(target_pointer_width = "32")]
    println!("{d:?}"); // Debug existe solo en 32 bits
    println!("{}", d.0);
}
