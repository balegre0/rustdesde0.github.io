fn main() {
    // cfg! pregunta por la plataforma para la que se COMPILÓ el programa (el «target»).
    println!("procesador: {}", std::env::consts::ARCH);
    if cfg!(target_os = "windows") {
        println!("compilado para Windows");
    } else if cfg!(target_os = "linux") {
        println!("compilado para Linux");
    } else if cfg!(target_os = "wasi") {
        println!("compilado para WASI (lo que usa el navegador de este curso)");
    } else {
        println!("compilado para otro sistema");
    }
    println!("¿familia wasm? {}", cfg!(target_family = "wasm"));
    println!("bits de un puntero: {}", usize::BITS);
}
