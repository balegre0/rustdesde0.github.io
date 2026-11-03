#[repr(C, packed)]
struct Cabecera {
    tipo: u8,
    largo: u32, // desalineado: empieza en el byte 1
}

static mut CONTADOR: u32 = 0;

fn main() {
    let c = Cabecera { tipo: 1, largo: 500 };
    // 1. `&raw const` crea un puntero crudo sin crear antes una referencia.
    //    Una referencia a un campo desalineado no está permitida; un puntero sí.
    let p = &raw const c.largo;
    // SAFETY: `p` apunta a `c.largo`, viva; read_unaligned no exige alineación.
    let largo = unsafe { p.read_unaligned() };
    println!("tipo {} largo {}", c.tipo, largo);
    // 2. `&raw mut` sobre un `static mut`: sin crear una referencia.
    let q = &raw mut CONTADOR;
    // SAFETY: un solo hilo y ninguna referencia viva a CONTADOR.
    unsafe { *q += 1 };
    println!("{}", unsafe { *q });
    // 3. Fuera de `&raw const` / `&raw mut`, `raw` es un nombre normal.
    let raw = 3;
    println!("{raw}");
}
