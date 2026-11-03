fn main() {
    // cfg!(feature = "...") es true solo si Cargo activó esa feature al compilar.
    if cfg!(feature = "colores") {
        println!("\x1b[32mverde\x1b[0m");
    } else {
        println!("sin colores");
    }
    println!("features activas: {}", if cfg!(feature = "colores") { "colores" } else { "ninguna" });
}
