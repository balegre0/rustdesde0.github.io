fn main() {
    // SAFETY: el programa tiene un solo hilo en este punto.
    unsafe { std::env::set_var("MODO", "prueba") };
    println!("{:?}", std::env::var("MODO"));
}
