#[repr(C, packed)]
struct Cabecera {
    tipo: u8,
    largo: u32,
}

fn main() {
    let c = Cabecera { tipo: 1, largo: 500 };
    // Una referencia a un campo de un struct empaquetado podría estar desalineada.
    let r = &c.largo;
    println!("{} {}", c.tipo, r);
}
