// Campos de un struct empaquetado pueden estar desalineados.
#[repr(packed)]
struct Cabecera {
    tipo: u8,
    largo: u32, // empieza en el byte 1: ¡desalineado!
}

fn main() {
    let c = Cabecera { tipo: 1, largo: 500 };
    let r = &c.largo; // una referencia DEBE estar alineada
    println!("{} {}", c.tipo, r);
}
