fn main() {
    let lleno = true;
    // Las dos ramas producen el mismo tipo: &str
    let mensaje = if lleno { "100 %" } else { "vacío" };
    println!("{mensaje}");
}
