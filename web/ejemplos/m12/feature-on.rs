fn main() {
    // El mismo programa, compilado como si Cargo hubiera activado `colores`.
    if cfg!(feature = "colores") {
        println!("con colores (la feature está activa)");
    } else {
        println!("sin colores");
    }
    #[cfg(feature = "colores")]
    fn solo_con_colores() -> &'static str {
        "esta función solo existe con la feature"
    }
    #[cfg(feature = "colores")]
    println!("{}", solo_con_colores());
}
