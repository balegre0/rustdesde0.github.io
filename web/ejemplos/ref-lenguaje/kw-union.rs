// 1. Una union guarda UNO de sus campos, todos en la misma memoria.
#[repr(C)]
union Bits {
    entero: u32,
    real: f32,
}

fn main() {
    let b = Bits { real: 1.0 };
    // 2. Leer un campo es unsafe: el compilador no sabe cuál se escribió.
    // SAFETY: cualquier patrón de 32 bits es un u32 válido.
    let e = unsafe { b.entero };
    println!("{e:#010x}");
    println!("{} bytes", size_of::<Bits>());
    // 3. Fuera de una declaración, `union` es un nombre normal.
    let union = "palabra débil";
    println!("{union}");
}
