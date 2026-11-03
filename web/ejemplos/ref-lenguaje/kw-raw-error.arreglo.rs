#[repr(C, packed)]
struct Cabecera {
    tipo: u8,
    largo: u32,
}

fn main() {
    let c = Cabecera { tipo: 1, largo: 500 };
    // Copiar el campo (con llaves) o usar `&raw const` + read_unaligned.
    let largo = { c.largo };
    println!("{} {}", c.tipo, largo);
}
