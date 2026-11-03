struct MiCaja<T>(T);

fn main() {
    let c = MiCaja(10);
    // MiCaja no implementa Deref: * no sabe qué hacer con ella.
    let n = *c + 1;
    println!("{n}");
}
