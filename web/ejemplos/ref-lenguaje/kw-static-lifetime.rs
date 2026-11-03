use std::fmt::Debug;

// 1. Un literal de texto vive en el binario: su tipo es &'static str.
const NOMBRE: &'static str = "Ferris";

// 2. `T: 'static`: T no contiene préstamos de vida corta.
//    Un String cumple (es dueño de sus datos), aunque no viva para siempre.
fn guardar<T: Debug + 'static>(x: T) -> Box<dyn Debug> {
    Box::new(x)
}

// 3. Box::leak: convierte un valor del montón en una referencia 'static.
fn eterno(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    println!("{NOMBRE}");
    let caja = guardar(String::from("dueño de sus datos"));
    println!("{caja:?}");
    let e = eterno(format!("{}!", NOMBRE));
    println!("{e}");
    // 4. Fuera de un tiempo de vida, `static` es otra palabra (ver `static`).
}
