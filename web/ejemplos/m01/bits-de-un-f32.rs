// Un f32 son 32 bits: 1 de signo, 8 de exponente y 23 de mantisa.
fn mostrar(x: f32) {
    let bits = x.to_bits(); // los 32 bits, tal cual, como un u32
    let signo = bits >> 31;
    let exponente = (bits >> 23) & 0xff;
    let mantisa = bits & 0x7f_ffff;
    println!(
        "{x:>6} -> {:#010x} -> signo {signo} | exponente {:08b} ({}) | mantisa {:023b}",
        bits, exponente, exponente, mantisa
    );
}

fn main() {
    // 1. Unos cuantos números y sus bits
    mostrar(1.0);
    mostrar(2.0);
    mostrar(-2.0);
    mostrar(0.5);
    mostrar(0.1);

    // 2. De bits a número: el camino de vuelta
    let x = f32::from_bits(0x4049_0fdb);
    println!("0x40490fdb es {x}");

    // 3. 0.1 no se puede guardar exacto: con más decimales se ve
    println!("0.1 guardado de verdad: {:.20}", 0.1f32);
}
