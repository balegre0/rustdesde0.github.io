// Campos de un struct empaquetado pueden estar desalineados.
#[repr(packed)]
struct Cabecera {
    tipo: u8,
    largo: u32, // empieza en el byte 1: ¡desalineado!
}

fn main() {
    let c = Cabecera { tipo: 1, largo: 500 };
    let p = &raw const c.largo; // puntero crudo: puede estar desalineado
    // SAFETY: p apunta a un u32 inicializado; read_unaligned no exige alineación.
    let largo = unsafe { p.read_unaligned() };
    let tipo = c.tipo; // copiar el valor (no crear referencia) siempre vale
    println!("{tipo} {largo}");
    println!("tamaño = {} bytes", size_of::<Cabecera>());
}
