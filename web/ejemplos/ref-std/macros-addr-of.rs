#[repr(C, packed)]
struct Cabecera {
    tipo: u8,
    longitud: u32, // desalineado: empieza en el byte 1
}

fn main() {
    let c = Cabecera { tipo: 7, longitud: 1500 };
    // &c.longitud no compilaría (E0793): una referencia tiene que estar alineada.
    // &raw const crea un puntero crudo sin pasar por una referencia.
    let p: *const u32 = &raw const c.longitud;
    // SAFETY: p apunta a un u32 válido; read_unaligned no exige alineación.
    let l = unsafe { p.read_unaligned() };
    let t = c.tipo;
    println!("tipo {t}, longitud {l}");
    // La forma antigua, equivalente:
    let q = std::ptr::addr_of!(c.longitud);
    println!("mismo puntero: {}", p == q);
}
