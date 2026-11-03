fn main() {
    // 1. cfg!(debug_assertions) vale true al compilar en modo depuración (debug)
    //    y false al compilar en modo publicación (release).
    if cfg!(debug_assertions) {
        println!("Compilado en modo debug: rápido de compilar, con comprobaciones extra.");
    } else {
        println!("Compilado en modo release: optimizado para ir rápido.");
    }
}
