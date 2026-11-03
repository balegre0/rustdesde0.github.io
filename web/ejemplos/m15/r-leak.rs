// Devuelve un &'static str construido en ejecución, a costa de no liberarlo nunca.
fn nombre_global(n: u32) -> &'static str {
    let s = format!("jugador-{n}");
    Box::leak(s.into_boxed_str())
}

fn main() {
    let a = nombre_global(7);
    let b = nombre_global(42);
    println!("{a} y {b}");
}
